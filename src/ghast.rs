use std::error::Error;

use crate::phase2::{self, FlatIR, ParseError};
use crate::phase3;

/// Ghastのコードをインタプリタ実行する
pub fn exec(input: String) -> Result<(), Box<dyn Error>> {
    // 入力が空なら終了
    if input.trim().is_empty() {
        return Ok(());
    }

    let result = phase2::ghast().parse(input);
    match result {
        Ok(ghast) => {
            eprintln!("受理🎉 {:?}", ghast);
            let core_ast = phase3::convert_into_ghast(ghast);
            eprintln!("コア言語💎 {:?}", core_ast);

            let value = phase3::eval(&core_ast);
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
