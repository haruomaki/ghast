use std::error::Error;

mod corelang;
mod ghast;
mod operator;

use ghast::{Ghast, ParseError};

fn main() -> Result<(), Box<dyn Error>> {
    // 入力受け付け
    let input = pomprt::new(">> ").read()?;

    let result = ghast::ghast_master().parse(input);
    match result {
        Ok(ghast) => {
            eprintln!("受理🎉 {:?}", ghast);
            let core_ast = corelang::convert_into_core(ghast);
            eprintln!("コア言語💎 {:?}", core_ast);

            let value = corelang::eval(&core_ast);
            println!("評価結果: {:?}", value);

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
