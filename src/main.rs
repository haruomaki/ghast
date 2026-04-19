use std::error::Error;

mod corelang;
mod operator;
mod phase2;

use phase2::{FlatIR, ParseError};

fn main() -> Result<(), Box<dyn Error>> {
    // 入力受け付け
    let input = pomprt::new(">> ").read()?;

    let result = phase2::ghast_master().parse(input);
    match result {
        Ok(ghast) => {
            eprintln!("受理🎉 {:?}", ghast);
            let core_ast = corelang::convert_into_ghast(ghast);
            eprintln!("コア言語💎 {:?}", core_ast);

            let value = corelang::eval(&core_ast);
            println!("評価結果: {:?}", value);

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
