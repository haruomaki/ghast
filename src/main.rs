mod monad;
mod parser;

use parser::Parser;
use std::io::{self, Write};

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Expr {
    Hello,
    World,
}

fn main() {
    print!("入力: ");
    io::stdout().flush().unwrap();
    let input = {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");
        buf.trim().to_string()
    };

    let parser_master = mdo! {
        ab <- Parser::terminal('a') | Parser::terminal('b');
        pq <- Parser::terminal('p') | Parser::terminal('q');
        lr <- Parser::terminal('l') | Parser::terminal('r');
        => vec![ab, pq, lr]
    };

    match parser_master.parse(&input) {
        Ok(ast) => println!("受理🎉 {:?}", ast),
        Err(e) => println!("拒否 {:?}", e),
    }
}
