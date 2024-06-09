use corelang::CoreLang;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
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

fn translate(ctr: &CompileController, ast: CoreLang) {
    match ast {
        CoreLang::Apply(func, args) => match args.as_ref() {
            CoreLang::Tuple(args) => build_apply(ctr, *func, args),
            _ => panic!("CoreLang::Applyの右辺はタプルである必要があります"),
        },

        _ => panic!("工事中。o＠(・_・)＠o。"),
    };
}

fn build_apply(ctr: &CompileController, func: CoreLang, args: &[CoreLang]) {
    if let CoreLang::Symbol(fname) = func {
        if fname == "print" {
            if let CoreLang::Lit(Literal::I32(value)) = args[0] {
                print_num(ctr, value);
            } else {
                panic!("printの引数が数値ではありません");
            }
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
            .unwrap();
    } else {
        panic!("関数名の直接指定にしか対応していません");
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

fn print_num<'ctx>(ctr: &'ctx CompileController, value: i32) -> CallSiteValue<'ctx> {
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

    // printfの呼び出し
    builder
        .build_call(
            fun,
            &[
                format_str.as_pointer_value().into(),
                i32_type.const_int(value as u64, false).into(),
            ],
            "printf_result",
        )
        .unwrap()
}
