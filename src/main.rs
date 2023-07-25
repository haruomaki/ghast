use std::str::Chars;

#[derive(Debug)]
enum Expression {
    Terminal(char),
    Sequence(Box<Expression>, Box<Expression>),
}

struct Parser<'a> {
    str_iter: Chars<'a>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        Parser {
            str_iter: input.chars(),
        }
    }

    fn parse_expression(&mut self, expression: &Expression) -> bool {
        match expression {
            Expression::Terminal(c) => {
                if let Some(next_char) = self.str_iter.next() {
                    return next_char == *c;
                }
                false
            }
            Expression::Sequence(e1, e2) => {
                self.parse_expression(&e1) && self.parse_expression(&e2)
            }
        }
    }
}

fn main() {
    let input = "ab";
    let mut parser = Parser::new(input);
    let expression = Expression::Sequence(
        Box::new(Expression::Terminal('a')),
        Box::new(Expression::Terminal('b')),
    );

    if parser.parse_expression(&expression) {
        println!("成功");
    } else {
        println!("失敗");
    }
}
