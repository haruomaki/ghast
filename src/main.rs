use std::error::Error;

mod corelang;
mod operator;
mod phase2;

use phase2::{FlatIR, ParseError};

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数がある場合それをパース、無ければデモ動作
    let input = match std::env::args().nth(1) {
        Some(arg) if arg == "-i" => pomprt::new("❯ ").read()?,
        Some(arg) => arg,
        None => {
            eprintln!("Demo mode.");
            String::from("1 + 1")
        }
    };

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
