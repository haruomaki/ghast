use corelang::CoreLang;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::values::{CallSiteValue, FunctionValue};
use inkwell::AddressSpace;

use std::error::Error;

mod corelang;
mod ghast;
mod operator;

use ghast::{Ghast, Literal, ParseError};

/// Context, Module, Builderをひとまとめにするための構造体
#[derive(Debug)]
pub struct CompileController<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

impl<'ctx> CompileController<'ctx> {
    // コンストラクタ関数
    pub fn new(context: &'ctx Context, module_name: impl AsRef<str>) -> Self {
        CompileController {
            context,
            module: context.create_module(module_name.as_ref()),
            builder: context.create_builder(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // 入力受け付け
    let input = pomprt::new(">> ").read()?;

    let result = ghast::ghast_master().parse(input);
    match result {
        Ok(ghast) => {
            eprintln!("受理🎉 {:?}", ghast);
            let core_ast = corelang::convert_into_core(ghast);
            eprintln!("コア言語💎 {:?}", core_ast);

            let ir = build_main(core_ast).unwrap();
            print!("{}", ir);

            Ok(())
        }

        Err(e) => {
            if let ParseError::IncompleteParse(e) = &e {
                if let Some(ast) = e.downcast_ref::<Ghast>() {
                    eprintln!("途中まで {:?}", ast);
                }
            }

            Err(Box::new(e))
        }
    }
}

// https://yhara.jp/2019/06/09/inkwell-hi
fn build_main(ast: CoreLang) -> Result<String, Box<dyn Error>> {
    let context = Context::create();
    let ctr = CompileController::new(&context, "main");
    let module = &ctr.module;
    let builder = &ctr.builder;
    let i32_type = context.i32_type();

    // define i32 @main() {
    let main_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", main_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    translate(&ctr, ast);

    // ret i32 0
    builder.build_return(Some(&i32_type.const_int(0, false)))?;

    Ok(module.print_to_string().to_string())
}

// 一つのCoreLangをコンパイルし、レジスタを作り出す。いまのところレジスタを作るのはリテラルとprintのみ。
fn translate<'ctx>(ctr: &'ctx CompileController, ast: CoreLang) -> CallSiteValue<'ctx> {
    match ast {
        CoreLang::Apply(func, args) => match *args {
            CoreLang::Tuple(args) => build_apply(ctr, *func, args),
            _ => panic!("CoreLang::Applyの右辺はタプルである必要があります"),
        },
        CoreLang::Lit(literal) => embed_literal(ctr, literal),

        _ => panic!("工事中。o＠(・_・)＠o。"),
    }
}

fn build_apply<'ctx>(
    ctr: &'ctx CompileController,
    func: CoreLang,
    args: Vec<CoreLang>,
) -> CallSiteValue<'ctx> {
    if let CoreLang::Symbol(fname) = func {
        if fname == "print" {
            let arg = args.into_iter().next().unwrap(); // 1つ目の引数を抽出
            print_num(ctr, arg)
        } else if fname == "add" {
            panic!("addですね");
        } else {
            panic!("未知の関数名です: {}", fname);
        }
    } else if let CoreLang::Fn(param, body) = func {
        let lambda = create_lambda(ctr, param, *body);
        let i32_type = ctr.context.i32_type();

        // ラムダの呼び出し
        ctr.builder
            .build_call(
                lambda,
                &[i32_type.const_int(0, false).into()],
                "lambda_result",
            )
            .unwrap()
    } else {
        panic!("Applyの左辺は非対応の種類です");
    }
}

fn create_lambda<'ctx>(
    ctr: &'ctx CompileController,
    _param: String,
    body: CoreLang,
) -> FunctionValue<'ctx> {
    // ラムダ式を実体化

    let builder = ctr.context.create_builder();
    let i32_type = ctr.context.i32_type();

    if let CoreLang::Lit(Literal::I32(value)) = body {
        // define i32 @main() {
        let func_type = i32_type.fn_type(&[], false);
        let function = ctr.module.add_function("lambda", func_type, None);
        let basic_block = ctr.context.append_basic_block(function, "");
        builder.position_at_end(basic_block);

        builder
            .build_return(Some(&i32_type.const_int(value as u64, false)))
            .unwrap();

        function
    } else {
        panic!("定数関数以外のラムダ式はまだサポートしていません");
    }
}

// 一引数のprintfの呼び出しを生成
fn print_num<'ctx>(ctr: &'ctx CompileController, arg: CoreLang) -> CallSiteValue<'ctx> {
    let module = &ctr.module;
    let builder = &ctr.builder;
    let i32_type = ctr.context.i32_type();
    let ptr_type = ctr.context.ptr_type(AddressSpace::default());

    let format_str = module.get_global("num_format").unwrap_or_else(|| {
        // 初回時に文字列リテラルをグローバル変数として追加
        builder
            .build_global_string_ptr("数値は%dです\n", "num_format")
            .unwrap()
    });

    let fun = module.get_function("printf").unwrap_or_else(|| {
        // declare i32 @printf(ptr, ...)
        let printf_type = i32_type.fn_type(&[ptr_type.into()], true);
        module.add_function("printf", printf_type, None)
    });

    let arg_site = translate(&ctr, arg);

    // printfの呼び出し
    builder
        .build_call(
            fun,
            &[
                format_str.as_pointer_value().into(),
                arg_site.try_as_basic_value().left().unwrap().into(),
            ],
            "printf_result",
        )
        .unwrap()
}

fn embed_literal<'ctx>(ctr: &'ctx CompileController, literal: Literal) -> CallSiteValue<'ctx> {
    match literal {
        Literal::I32(value) => {
            let module = &ctr.module;
            let builder = &ctr.builder;
            let i32_type = ctr.context.i32_type();
            let fn_type = i32_type.fn_type(&[i32_type.into()], false);

            let idfunc = module.get_function("id_i32").unwrap_or_else(|| {
                // 恒等関数「id_i32」を生成
                let function: FunctionValue =
                    module.add_function("id_i32", fn_type, Some(Linkage::Private));
                let entry = ctr.context.append_basic_block(function, "");
                let builder = ctr.context.create_builder();
                builder.position_at_end(entry);
                let x_value = function.get_first_param().unwrap().into_int_value(); // 引数を取得しそのまま返す
                builder.build_return(Some(&x_value)).unwrap();
                function
            });

            // 「id_i32」の呼び出し
            builder
                .build_call(
                    idfunc,
                    &[i32_type.const_int(value as u64, false).into()],
                    "litc",
                )
                .unwrap()
        } // _ => panic!("I32以外のリテラルの埋め込みは未実装です"),
    }
}
