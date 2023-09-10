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

    // let parser_master = mdo! {
    //     _ <- Parser::terminal('0');
    //     prefix <- Parser::terminal('7') | Parser::terminal('8') | Parser::terminal('9');
    //     _ <- Parser::terminal('0');
    //     _ <- Parser::terminal('-');
    //     region <- Parser::ascii_digit().many(Some(4), Some(4));
    //     _ <- Parser::terminal('-');
    //     id <- Parser::ascii_digit().many(Some(4), Some(4));
    //     => prefix
    // };
    let parser_head = pdo! {
        _ <- Parser::terminal('0');
        prefix <- Parser::terminal('7') | Parser::terminal('8') | Parser::terminal('9');
        _ <- Parser::terminal('0');
        => vec!['0', prefix, '0']
    };

    let parser_master = parser_head.bind(move |region| {
        let region = region.clone();
        Parser::terminal('-').bind(move |_| {
            let region = region.clone();
            Parser::ascii_digit().bind(move |_| Parser::ret(region))
        })
    });

    let a = 334;
    let b = 334;
    pdo_with_env! (~a b~
        x <- Parser::terminal('q');
        => (x, a+b)
    );

    pdo_with_env! {~ a ~
        => 'A'
    };

    let _parser_pdo = pdo_with_env! {~~
        one <- Parser::terminal('0');
        prefix <- Parser::terminal('7') | Parser::terminal('8') | Parser::terminal('9');
        two <- Parser::terminal('0');
        => (one, prefix, two)
    };

    match parser_master.parse(&input) {
        Ok(ast) => println!("受理🎉 {:?}", ast),
        Err(e) => println!("拒否 {:?}", e),
    }
}
