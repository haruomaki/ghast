mod monad;
mod parser;

use parser::Parser;

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Expr {
    Hello,
    World,
}

fn consume(it: &mut std::str::Chars) {
    let c = it.next().unwrap();
    println!("{}", c);
}

fn main() {
    let input = "a";

    let parser_a = Parser::terminal('a');
    let parser_b = Parser::terminal('b');

    let parser_master = Parser::choice(parser_a, parser_b);
    let result = parser_master.parse(input);
    println!("{:?}", result);
}
