use std::error::Error;

use ghast::phase2::{self, FlatIR, ParseError};

fn parse_and_print(input: String) -> Result<(), Box<dyn Error>> {
    // 入力が空なら終了
    if input.trim().is_empty() {
        return Ok(());
    }

    let result = phase2::ghast().parse(input);
    match result {
        Ok(ghast) => {
            eprintln!("受理🎉 {:?}", ghast);
            // let core_ast = corelang::convert_into_ghast(ghast);
            // eprintln!("コア言語💎 {:?}", core_ast);

            // let value = corelang::eval(&core_ast);
            // println!("評価結果: {:?}", value);

            Ok(())
        }

        Err(e) => {
            if let ParseError::IncompleteParse(e) = &e {
                if let Some(ast) = e.downcast_ref::<FlatIR>() {
                    eprintln!("途中まで {:?}", ast);
                }
            }

            Err(Box::new(e))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数がある場合それをパース、無ければデモ動作
    match std::env::args().nth(1) {
        Some(arg) if arg == "-i" => {
            for input in pomprt::new(">>> ") {
                if let Err(e) = parse_and_print(input) {
                    println!("{:?}", e);
                }
            }
            Ok(())
        }
        Some(arg) => parse_and_print(arg),
        None => {
            // デモモード
            eprintln!("Demo mode.");
            parse_and_print(String::from("1 + 1"))
        }
    }
}
