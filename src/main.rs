use std::error::Error;

use ghast::exec;

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数をパース
    let args: Vec<String> = std::env::args().collect();

    // 引数なし（デモ）
    if args.len() == 1 {
        eprintln!("Demo mode.");
        exec(String::from("1 + 1"), true)?;
    }
    // ヘルプを表示
    else if args[1] == "--help" {
        eprintln!("Usage: ghast <file> | -i | -c <command>");
        eprintln!("  <file>    : Execute file");
        eprintln!("  -i        : Start REPL");
        eprintln!("  -c ...    : Execute command");
        eprintln!("  (no args) : Demo mode");
    }
    // REPL 起動
    else if args[1] == "-i" {
        for input in pomprt::new(">>> ") {
            match exec(input, false) {
                Ok(v) => println!("{}", v),
                Err(e) => println!("{:?}", e),
            }
        }
    }
    // コマンド実行 (-c 引数)
    else if args[1] == "-c" {
        if args.len() <= 2 {
            return Err("コマンドが指定されていません".to_string().into());
        }
        let cmd = args[2].clone();
        exec(cmd, true)?;
    }
    // ファイル実行
    else {
        let filename = args[1].clone();
        let code = std::fs::read_to_string(&filename)?;
        exec(code, false)?;
    }

    Ok(())
}
