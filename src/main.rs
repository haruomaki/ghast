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
    let input = "Hello, World!";

    let parser_master = mdo! {
        _large_h <- Parser::terminal('H');
        _small_e <- Parser::terminal('e');
        _small_l1 <- Parser::terminal('l');
        _small_l2 <- Parser::terminal('l');
        _small_o <- Parser::terminal('o');
        => Expr::Hello
    };
    let result = parser_master.parse(input);
    println!("{:?}", result);
}
