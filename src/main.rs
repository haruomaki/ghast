mod operators;
mod parser;
mod utils;

use parser::Parser;
use std::io::{self, Write};
use utils::*;

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

    let pv = Parser::terminal('A') & Parser::terminal('a');
    let pw = Parser::terminal('A') & Parser::terminal('a');
    let pa = pv & Parser::terminal('b');
    let pb = Parser::terminal('b') & pw;
    // let pc = pv.concat(pw);

    let parser_master = pdo! {
        Parser::chunk("phone:");
        single('0');
        prefix <- Parser::terminal('7') | Parser::terminal('8') | Parser::terminal('9');
        Parser::terminal('0');
        Parser::terminal('-');
        region <- ascii_digit() * (4..5);
        Parser::terminal('-');
        id <- ascii_digit() * (4..5);
        return (prefix, region, id)
    };

    match parser_master.parse(&input) {
        Ok(ast) => println!("受理🎉 {:?}", ast),
        Err(e) => println!("拒否 {:?}", e),
    }
}
