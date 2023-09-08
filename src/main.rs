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
        _ <- Parser::terminal('0');
        prefix <- Parser::terminal('7') | Parser::terminal('8') | Parser::terminal('9');
        _ <- Parser::terminal('0');
        _ <- Parser::terminal('-');
        region <- Parser::ascii_digit().many(Some(4), Some(4));
        _ <- Parser::terminal('-');
        id <- Parser::ascii_digit().many(Some(4), Some(4));
        => prefix
    };

    match parser_master.parse(&input) {
        Ok(ast) => println!("受理🎉 {:?}", ast),
        Err(e) => println!("拒否 {:?}", e),
    }
}
