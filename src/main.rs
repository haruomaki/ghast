use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
enum ParseError {
    WrongTerminal,
    MissingNonTerminal,
    IncompleteParse,
}

type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
enum AST {
    Terminal(char),
    NonTerminal(String, Box<AST>),
    Sequence(Box<AST>, Box<AST>),
}

#[derive(Debug)]
enum Expression {
    Terminal(char),
    NonTerminal(String),
    Sequence(Box<Expression>, Box<Expression>),
    OrderedChoice(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn parse(&self, grammar: &Grammar, input: &str, i: usize) -> ParseResult<(usize, AST)> {
        let result = match self {
            Expression::Terminal(c) => {
                if input.chars().nth(i) == Some(*c) {
                    Ok((i + 1, AST::Terminal(*c)))
                } else {
                    Err(ParseError::WrongTerminal)
                }
            }
            Expression::NonTerminal(symbol) => {
                let expr = grammar
                    .rules
                    .get(symbol)
                    .ok_or(ParseError::MissingNonTerminal)?;
                let (j, ast) = expr.parse(grammar, input, i)?;
                Ok((j, AST::NonTerminal(symbol.clone(), Box::new(ast))))
            }
            Expression::Sequence(e1, e2) => {
                let (j, ast1) = e1.parse(grammar, input, i)?;
                let (k, ast2) = e2.parse(grammar, input, j)?;
                Ok((k, AST::Sequence(Box::new(ast1), Box::new(ast2))))
            }
            Expression::OrderedChoice(e1, e2) => match e1.parse(grammar, input, i) {
                Ok((j, ast)) => Ok((j, ast)),
                Err(_) => e2.parse(grammar, input, i),
            },
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

    fn parse(&self, input: &str) -> ParseResult<AST> {
        let master_expr = Expression::NonTerminal(self.start_symbol.clone());
        let (j, ast) = master_expr.parse(self, input, 0)?;
        if j == input.len() {
            Ok(ast)
        } else {
            Err(ParseError::IncompleteParse)
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

    match grammar.parse(&input) {
        Ok(ast) => println!("受理🎉 {:?}", ast),
        Err(e) => println!("拒否({:?})", e),
    }
}
