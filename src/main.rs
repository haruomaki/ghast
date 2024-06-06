use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::AddressSpace;

use monapa::*;
use std::error::Error;
use std::io::{self, Write};

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Ghast {
    Symbol(String),
    Fn(Box<Ghast>, Box<Ghast>),
    Apply(Box<Ghast>, Box<Ghast>),
    I32(i32),
}

fn id_start() -> Parser<char> {
    Parser::satisfy(|c| c == '_' || c.is_alphabetic())
}

fn id_continue() -> Parser<char> {
    Parser::satisfy(|c| c == '_' || c.is_alphanumeric())
}

fn literal_digit() -> Parser<char> {
    Parser::satisfy(|c| c.is_ascii_digit())
}

fn ghast_symbol() -> Parser<Ghast> {
    pdo! {
        start <- id_start();
        conti <- id_continue() * ..;
        let idvec = vec![vec![start], conti].concat();
        return Ghast::Symbol(idvec.iter().collect())
    }
}

fn ghast_fn() -> Parser<Ghast> {
    pdo! {
        single('\\');
        arg <- ghast_symbol();
        whitespace() * (..);
        chunk("->");
        whitespace() * (..);
        cont <- ghast_master();
        return Ghast::Fn(Box::new(arg), Box::new(cont))
    }
}

fn ghast_i32() -> Parser<Ghast> {
    pdo! {
        num <- literal_digit() * (1..);
        let num_str = num.iter().collect::<String>();
        return Ghast::I32(num_str.parse().unwrap())
    }
}

fn ghast_master() -> Parser<Ghast> {
    ghast_fn() | ghast_symbol() | ghast_i32()
}

fn main() {
    eprint!("入力: ");
    io::stdout().flush().unwrap();
    let input = {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");
        buf.trim().to_string()
    };

    let parser_master = ghast_master();

    match parser_master.parse(&input) {
        Ok(ast) => eprintln!("受理🎉 {:?}", ast),
        Err(e) => {
            eprintln!("拒否 {:?}", e);
            if let ParseError::IncompleteParse(e) = &e {
                if let Some(ast) = e.downcast_ref::<Ghast>() {
                    eprintln!("途中まで {:?}", ast);
                }
            }
        }
    }

    let ir = build_main().unwrap();
    print!("{}", ir);
}

// https://yhara.jp/2019/06/09/inkwell-hi
fn build_main() -> Result<String, Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i32_type = context.i32_type();

    // define i32 @main() {
    let main_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", main_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    print_num(&context, &module, &builder, 334);

    // ret i32 0
    builder.build_return(Some(&i32_type.const_int(0, false)))?;

    Ok(module.print_to_string().to_string())
}

fn print_num<'ctx>(context: &'ctx Context, module: &Module<'ctx>, builder: &Builder, value: i32) {
    let i32_type = context.i32_type();
    let i8_type = context.i8_type();
    let i8ptr_type = i8_type.ptr_type(AddressSpace::default());

    let format_str = module.get_global("num_format").unwrap_or_else(|| {
        // 初回時に文字列リテラルをグローバル変数として追加
        builder
            .build_global_string_ptr("数値は%dです\n", "num_format")
            .unwrap()
    });

    let fun = module.get_function("printf").unwrap_or_else(|| {
        // declare i32 @printf(ptr, ...)
        let printf_type = i32_type.fn_type(&[i8ptr_type.into()], true);
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
        .unwrap();
}
