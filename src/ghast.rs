use std::error::Error;

use crate::phase2::{self, FlatIR, ParseError};
use crate::phase3::{self, Value};

/// Ghastのコードをインタプリタ実行する
pub fn exec(input: String, debug: bool) -> Result<Value, Box<dyn Error>> {
    // 入力が空なら終了
    if input.trim().is_empty() {
        return Ok(Value::Unit);
    }

    let result = phase2::ghast().parse(input);
    match result {
        Ok(ghast) => {
            if debug {
                eprintln!("受理🎉 {:?}", ghast);
            }

            let core_ast = phase3::convert_into_ghast(ghast);
            if debug {
                eprintln!("コア言語💎 {:?}", core_ast);
            }

            let value = phase3::eval(&core_ast);
            if debug {
                eprintln!("評価結果: {:?}", value);
            }

            Ok(value)
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
