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

    let parser_symbol = (Parser::satisfy(|c| !char::is_whitespace(c)) * (1..))
        .map(|s| Ghast::Symbol(s.iter().collect()));
    let whitespaces = whitespace() * (1..);

    let parser_master = parser_symbol.clone().bind(move |head| {
        let head = head.clone();
        let parser_symbol = parser_symbol.clone();
        (whitespaces.clone().bind(move |_| parser_symbol.clone()) * (..)).bind(move |tail| {
            let head = head.clone();
            Parser::ret(vec![vec![head.clone()], tail].concat())
        })
    });

    match parser_master.parse(&input) {
        Ok(ast) => println!("受理🎉 {:?}", ast),
        Err(e) => println!("拒否 {:?}", e),
    }
}
