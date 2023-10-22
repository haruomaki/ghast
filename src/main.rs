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

fn id_start() -> Parser<char> {
    alphabetic()
}

fn id_continue() -> Parser<char> {
    alphanumeric()
}

fn ghast_symbol() -> Parser<Ghast> {
    pdo! {
        start <- id_start();
        conti <- id_continue() * ..;
        let idvec = vec![vec![start], conti].concat();
        return Ghast::Symbol(idvec.iter().collect())
    }
}

fn ghast_fn() -> Parser<Ghast> {
    pdo! {
        single('\\');
        arg <- ghast_symbol();
        whitespace() * (..);
        chunk("->");
        whitespace() * (..);
        cont <- ghast_master();
        return Ghast::Fn(Box::new(arg), Box::new(cont))
    }
}

fn ghast_master() -> Parser<Ghast> {
    ghast_fn() | ghast_symbol()
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

    let parser_master = ghast_master();

    match parser_master.parse(&input) {
        Ok(ast) => println!("受理🎉 {:?}", ast),
        Err(e) => {
            println!("拒否 {:?}", e);
            if let ParseError::IncompleteParse(e) = &e {
                if let Some(ast) = e.downcast_ref::<Ghast>() {
                    println!("途中まで {:?}", ast);
                }
            }
        }
    }
}
