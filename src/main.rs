use monapa::*;
use std::io::{self, Write};

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Ghast {
    Symbol(String),
    List(Vec<Ghast>),
    Fn(Box<Ghast>, Box<Ghast>),
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

    let parser_master = Parser::recurse(|parser_master: Parser<Vec<char>>| {
        pdo! (
            single('(');
            p <- parser_master.clone();
            single(')');
            return vec![vec!['('], p, vec![')']].concat()
        ) | single('a').map(|a| vec![a])
    });

    match parser_master.parse(&input) {
        Ok(ast) => println!("受理🎉 {:?}", ast),
        Err(e) => println!("拒否 {:?}", e),
    }
}
