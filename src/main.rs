mod monad;
mod parser;

use parser::Parser;

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Expr {
    Hello,
    World,
}

fn main() {
    let input = "apr";

    let parser_master = mdo! {
        ab <- Parser::terminal('a') | Parser::terminal('b');
        pl <- Parser::terminal('p') | Parser::terminal('l');
        qr <- Parser::terminal('q') | Parser::terminal('r');
        => vec![ab, pl, qr]
    };
    let result = parser_master.parse(input);
    println!("{:?}", result);
}
