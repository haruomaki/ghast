use monapa::*;
use std::io::{self, Write};

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Ghast {
    Symbol(String),
    Tuple(Vec<Ghast>),
    Fn(Box<Ghast>, Box<Ghast>),
    Apply(Box<Ghast>, Box<Ghast>),
}

fn symbol() -> Parser<Ghast> {
    (Parser::satisfy(|c| !char::is_whitespace(c)) * (1..))
        .map(|s| Ghast::Symbol(s.iter().collect()))
}

fn whitespaces() -> Parser<()> {
    (whitespace() * (1..)).map(|_| ())
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

    let parser_master = pdo! {
        head <- symbol();
        tail <- pdo! {
            whitespaces();
            symbol()
        } * (..);
        return vec![vec![head], tail].concat()
    };

    match parser_master.parse(&input) {
        Ok(ast) => println!("受理🎉 {:?}", ast),
        Err(e) => println!("拒否 {:?}", e),
    }
}
