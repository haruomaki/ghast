use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
enum Expression {
    Terminal(char),
    NonTerminal(String),
    Sequence(Box<Expression>, Box<Expression>),
    OrderedChoice(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn parse(&self, grammar: &Grammar, input: &str, i: usize) -> Option<usize> {
        let result = match self {
            Expression::Terminal(c) => {
                if input.chars().nth(i) == Some(*c) {
                    Some(i + 1)
                } else {
                    None
                }
            }
            Expression::NonTerminal(symbol) => {
                let expr = grammar.rules.get(symbol)?;
                expr.parse(grammar, input, i)
            }
            Expression::Sequence(e1, e2) => {
                let j = e1.parse(grammar, input, i)?;
                e2.parse(grammar, input, j)
            }
            Expression::OrderedChoice(e1, e2) => {
                if let Some(j) = e1.parse(grammar, input, i) {
                    Some(j)
                } else {
                    e2.parse(grammar, input, i)
                }
            }
        };

        result
    }
}

#[derive(Debug)]
struct Grammar {
    rules: HashMap<String, Expression>,
    start_symbol: String,
}

impl Grammar {
    fn new(start_symbol: &str) -> Self {
        Grammar {
            rules: HashMap::new(),
            start_symbol: start_symbol.to_string(),
        }
    }

    fn add_rule(&mut self, sym: &str, expr: Expression) {
        self.rules.insert(sym.to_string(), expr);
    }

    fn parse(&self, input: &str) -> bool {
        let master_expr = Expression::NonTerminal(self.start_symbol.clone());
        if let Some(t) = master_expr.parse(self, input, 0) {
            t == input.len()
        } else {
            false
        }
    }
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

    let mut grammar = Grammar::new("S");
    grammar.add_rule(
        "S",
        Expression::OrderedChoice(
            Box::new(Expression::Sequence(
                Box::new(Expression::NonTerminal("A".to_string())),
                Box::new(Expression::NonTerminal("B".to_string())),
            )),
            Box::new(Expression::Sequence(
                Box::new(Expression::NonTerminal("A".to_string())),
                Box::new(Expression::NonTerminal("A".to_string())),
            )),
        ),
    );
    grammar.add_rule("A", Expression::Terminal('a'));
    grammar.add_rule(
        "B",
        Expression::Sequence(
            Box::new(Expression::Terminal('a')),
            Box::new(Expression::Terminal('b')),
        ),
    );

    if grammar.parse(&input) {
        println!("受理🎉");
    } else {
        println!("拒否");
    }
}
