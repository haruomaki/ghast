use std::error::Error;

use ghast::exec;

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数がある場合それをパース、無ければデモ動作
    match std::env::args().nth(1) {
        Some(arg) if arg == "-i" => {
            for input in pomprt::new(">>> ") {
                match exec(input, false) {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Some(arg) => {
            exec(arg, true)?;
        }
        None => {
            // デモモード
            eprintln!("Demo mode.");
            exec(String::from("1 + 1"), true)?;
        }
    };
    Ok(())
}
