use corelang::CoreLang;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{AnyValue, AnyValueEnum, BasicValueEnum, FunctionValue};
use inkwell::AddressSpace;

use std::error::Error;
use std::rc::Rc;

mod corelang;
mod ghast;
mod operator;

use ghast::{Ghast, Literal, ParseError};

/// Context, Module, Builderをひとまとめにするための構造体
#[derive(Debug)]
pub struct CompileController<'ctx> {
    pub context: &'ctx Context,
    pub module: Rc<Module<'ctx>>,
    pub builder: Rc<Builder<'ctx>>,
}

impl<'ctx> CompileController<'ctx> {
    // コンストラクタ関数
    pub fn new(context: &'ctx Context, module_name: impl AsRef<str>) -> Self {
        CompileController {
            context,
            module: Rc::new(context.create_module(module_name.as_ref())),
            builder: Rc::new(context.create_builder()),
        }
    }

    pub fn with(&self, builder: Rc<Builder<'ctx>>) -> Self {
        CompileController {
            context: self.context,
            module: self.module.clone(),
            builder,
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
fn translate<'ctx>(ctr: &'ctx CompileController, ast: CoreLang) -> AnyValueEnum<'ctx> {
    match ast {
        CoreLang::Symbol(name) => match name.as_str() {
            "print" => create_print(ctr).as_any_value_enum(),
            "add" => panic!("addですね"),
            _ => panic!("未知のシンボルです"),
        },
        CoreLang::Fn(param, body) => create_lambda(ctr, param, *body).as_any_value_enum(),
        CoreLang::Apply(func, args) => match *args {
            CoreLang::Tuple(args) => build_apply(ctr, *func, args),
            _ => panic!("CoreLang::Applyの右辺はタプルである必要があります"),
        },
        CoreLang::Lit(literal) => embed_literal(ctr, literal),
        CoreLang::Tuple(elems) => build_tuple(ctr, elems),
    }
}

fn build_apply<'ctx>(
    ctr: &'ctx CompileController,
    func: CoreLang,
    args: Vec<CoreLang>,
) -> AnyValueEnum<'ctx> {
    let fvalue = translate(ctr, func);

    let arg = args
        .into_iter()
        .next()
        .expect("関数適用は1引数だけ対応です");
    let avalue = translate(ctr, arg);

    if let AnyValueEnum::FunctionValue(f) = fvalue {
        let ret = ctr
            .builder
            .build_call(
                f,
                &[avalue.try_into().expect("Applyの左辺が無効値です")],
                "lambda_result",
            )
            .unwrap()
            .try_as_basic_value();

        let nil_type = ctr.context.struct_type(&[], false);
        let nil_value = nil_type.const_named_struct(&[]);
        ret.left_or(nil_value.into()).as_any_value_enum()
    } else {
        panic!("Applyの左辺がFunctionValueでありません");
    }

    // if let CoreLang::Symbol(fname) = func {
    //     if fname == "print" {
    //         let arg = args.into_iter().next().unwrap(); // 1つ目の引数を抽出
    //         print_num(ctr, arg)
    //     } else if fname == "add" {
    //         panic!("addですね");
    //     } else {
    //         panic!("未知の関数名です: {}", fname);
    //     }
    // } else if let CoreLang::Fn(param, body) = func {
    //     let lambda = create_lambda(ctr, param, *body);
    //     let i32_type = ctr.context.i32_type();
}

fn create_lambda<'ctx>(
    ctr: &'ctx CompileController,
    _param: String,
    body: CoreLang,
) -> FunctionValue<'ctx> {
    // ラムダ式を実体化

    let builder = Rc::new(ctr.context.create_builder());
    let i32_type = ctr.context.i32_type();

    // define i32 @lambda() {
    let func_type = i32_type.fn_type(&[], false);
    let function = ctr
        .module
        .add_function("lambda", func_type, Some(Linkage::Private));
    let basic_block = ctr.context.append_basic_block(function, "");
    builder.position_at_end(basic_block);

    let inner_ctr = ctr.with(builder.clone());
    let body_site = translate(&inner_ctr, body);

    let ret: BasicValueEnum = body_site.try_into().expect("関数本体の型が不正です");
    builder.build_return(Some(&ret)).unwrap();

    function
}

// 整数を1つ表示するprint関数を生成
fn create_print<'ctx>(ctr: &'ctx CompileController) -> FunctionValue<'ctx> {
    let module = &ctr.module;
    let builder = &ctr.context.create_builder();
    let i32_type = ctr.context.i32_type();
    let void_type = ctr.context.void_type();
    let ptr_type = ctr.context.ptr_type(AddressSpace::default());

    if let Some(f) = module.get_function("print") {
        return f;
    }

    // define i32 @print(i32) { ...
    let func_type = void_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("print", func_type, Some(Linkage::Private));
    let basic_block = ctr.context.append_basic_block(function, "");
    builder.position_at_end(basic_block);
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

    let v = function.get_first_param().unwrap().into_int_value();

    // printfの呼び出し
    builder
        .build_call(
            fun,
            &[format_str.as_pointer_value().into(), v.into()],
            "printf_result",
        )
        .unwrap()
        .try_as_basic_value()
        .unwrap_left()
        .as_any_value_enum();

    builder.build_return(None).unwrap();

    function
}

fn embed_literal<'ctx>(ctr: &'ctx CompileController, literal: Literal) -> AnyValueEnum<'ctx> {
    match literal {
        Literal::I32(value) => {
            let i32_type = ctr.context.i32_type();
            i32_type.const_int(value as u64, false).as_any_value_enum()
        } // _ => panic!("I32以外のリテラルの埋め込みは未実装です"),
    }
}

fn build_tuple<'ctx>(ctr: &'ctx CompileController, elems: Vec<CoreLang>) -> AnyValueEnum<'ctx> {
    let rets: Vec<BasicValueEnum> = elems
        .into_iter()
        .map(|elem| {
            let any_value = translate(ctr, elem);
            let basic_value = any_value
                .try_into()
                .expect("タプルの要素がBasicValueに変換できません");
            basic_value
        })
        .collect();
    let types: Vec<BasicTypeEnum> = rets.iter().map(|r| r.get_type()).collect();

    let tuple_type = ctr.context.struct_type(&types, false);
    let tuple_value = tuple_type.const_named_struct(&rets);
    tuple_value.as_any_value_enum()
}
