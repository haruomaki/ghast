//! Ghastの組み込み関数を定義するモジュール

use crate::phase3::{Env, Value};

/// 環境に組み込み関数を登録する
pub fn register(env: &mut Env) {
    env.insert(String::from("add"), Value::Builtin("add"));
    env.insert(String::from("sub"), Value::Builtin("sub"));
    env.insert(String::from("mul"), Value::Builtin("mul"));
    env.insert(String::from("div"), Value::Builtin("div"));
    env.insert(String::from("eq"), Value::Builtin("eq"));
    env.insert(String::from("neg"), Value::Builtin("neg"));
    env.insert(String::from("pos"), Value::Builtin("pos"));
    env.insert(String::from("not"), Value::Builtin("not"));
    env.insert(String::from("input"), Value::Builtin("input"));
    env.insert(String::from("print"), Value::Builtin("print"));
}

/// Ghast の組み込み関数を呼び出します
///
/// ## パラメータ
/// - `name`: 呼び出す組み込み関数名 (例: "add", "sub", "mul", "div", "neg", "pos", "not", "print")
/// - `args_value`: 関数に渡す引数の Value (通常は 2 つの要素を含む Tuple)
///
/// ## 戻り値
/// 呼び出された関数の戻り値 (Value)
pub fn invoke(name: &str, args_value: Value) -> Value {
    match name {
        "add" => match args_value {
            Value::Tuple(mut elements) if elements.len() == 2 => {
                let rhs = elements.pop().unwrap().as_i32();
                let lhs = elements.pop().unwrap().as_i32();
                Value::I32(lhs + rhs)
            }
            _ => panic!("add は 2 つの整数を取ります"),
        },
        "sub" => match args_value {
            Value::Tuple(mut elements) if elements.len() == 2 => {
                let rhs = elements.pop().unwrap().as_i32();
                let lhs = elements.pop().unwrap().as_i32();
                Value::I32(lhs - rhs)
            }
            _ => panic!("sub は 2 つの整数を取ります"),
        },
        "mul" => match args_value {
            Value::Tuple(mut elements) if elements.len() == 2 => {
                let rhs = elements.pop().unwrap().as_i32();
                let lhs = elements.pop().unwrap().as_i32();
                Value::I32(lhs * rhs)
            }
            _ => panic!("mul は 2 つの整数を取ります"),
        },
        "div" => match args_value {
            Value::Tuple(mut elements) if elements.len() == 2 => {
                let rhs = elements.pop().unwrap().as_i32();
                let lhs = elements.pop().unwrap().as_i32();
                Value::I32(lhs / rhs)
            }
            _ => panic!("div は 2 つの整数を取ります"),
        },
        "eq" => match args_value {
            Value::Tuple(mut elements) if elements.len() == 2 => {
                let rhs = elements.pop().unwrap().as_i32();
                let lhs = elements.pop().unwrap().as_i32();
                Value::Bool(lhs == rhs)
            }
            _ => panic!("eq は 2 つの整数を取ります"),
        },
        "neg" => match args_value {
            Value::I32(value) => Value::I32(-value),
            _ => panic!("neg は 1 つの整数を取ります"),
        },
        "pos" => match args_value {
            Value::I32(value) => Value::I32(value),
            _ => panic!("pos は 1 つの整数を取ります"),
        },
        "not" => match args_value {
            Value::I32(value) => Value::I32(if value != 0 { 0 } else { 1 }),
            _ => panic!("not は 1 つの整数を取ります"),
        },
        "input" => {
            // プロンプトを表示
            use std::io::Write;
            if let Value::Tuple(vals) = args_value {
                print!("{}", vals[0]);
                std::io::stdout().flush().unwrap(); // ここで flush
            }

            // 標準入力から文字列を取得
            let input = {
                let mut buf = String::new();
                std::io::stdin()
                    .read_line(&mut buf)
                    .expect("Failed to read from stdin");
                buf.trim().to_string()
            };
            let value: i32 = input.parse().expect("Failed to parse as i32");
            Value::I32(value)
        }
        "print" => {
            // TODO: 関数には必ずタプルが渡されることにしているが、ちょっと処理がめんどい、、
            match args_value {
                Value::Tuple(vals) => {
                    // 空白区切りで出力
                    println!(
                        "{}",
                        vals.iter()
                            .map(|v| format!("{}", v))
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                }
                _ => panic!("関数への入力がタプルでありません"),
            }
            Value::Unit
        }
        _ => panic!("未知の組み込み関数です: {}", name),
    }
}
