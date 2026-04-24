use std::error::Error;

use ghast::exec;

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数がある場合それをパース、無ければデモ動作
    match std::env::args().nth(1) {
        Some(arg) if arg == "-i" => {
            for input in pomprt::new(">>> ") {
                if let Err(e) = exec(input) {
                    println!("{:?}", e);
                }
            }
            Ok(())
        }
        Some(arg) => exec(arg),
        None => {
            // デモモード
            eprintln!("Demo mode.");
            exec(String::from("1 + 1"))
        }
    }
}
