// #[derive(Debug)]
// enum ParseStatus {
//     Continue,
//     Accept,
//     Reject,
// }

#[derive(Debug)]
enum Expression {
    Terminal(char),
    NonTerminal(String),
    Sequence(Box<Expression>, Box<Expression>),
    OrderedChoice(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn parse(&self, grammar: &Grammar, str: &String, i: usize) -> Option<usize> {
        println!("parse: {:?}, i = {}", self, i);
        let result = match self {
            Expression::Terminal(c) => {
                if str.chars().nth(i) == Some(*c) {
                    Some(i + 1)
                } else {
                    None
                }
            }
            Expression::NonTerminal(sym) => grammar.parse_symbol(sym, str, i),
            Expression::Sequence(e1, e2) => {
                if let Some(j) = e1.parse(grammar, str, i) {
                    e2.parse(grammar, str, j)
                } else {
                    None
                }
            }
            Expression::OrderedChoice(e1, e2) => {
                if let Some(j) = e1.parse(grammar, str, i) {
                    Some(j)
                } else {
                    e2.parse(grammar, str, i)
                }
            }
        };

        println!("end: {:?}, i = {}, result = {:?}", self, i, result);
        result
    }
}

#[derive(Debug)]
struct Grammar {
    rules: Vec<(String, Expression)>,
    start_symbol: String,
}

impl Grammar {
    fn new(start_symbol: &str) -> Self {
        Grammar {
            rules: Vec::new(),
            start_symbol: start_symbol.to_string(),
        }
    }

    fn add_rule(&mut self, sym: &str, expr: Expression) {
        self.rules.push((sym.to_string(), expr));
    }

    fn parse(&self, str: &String) -> bool {
        if let Some(t) = self.parse_symbol(&self.start_symbol, str, 0) {
            t == str.len()
        } else {
            false
        }
    }

    fn parse_symbol(&self, symbol: &str, str: &String, i: usize) -> Option<usize> {
        if let Some(expr) = self
            .rules
            .iter()
            .find(|(sym, _)| *sym == symbol)
            .map(|(_, e)| e)
        {
            expr.parse(self, str, i)
        } else {
            None
        }
    }
}

fn main() {
    let str = String::from("aab");

    let mut grammar = Grammar::new("S");
    grammar.add_rule(
        "S",
        Expression::OrderedChoice(
            Box::new(Expression::Sequence(
                Box::new(Expression::NonTerminal("A".to_string())),
                Box::new(Expression::NonTerminal("A".to_string())),
            )),
            Box::new(Expression::Sequence(
                Box::new(Expression::NonTerminal("A".to_string())),
                Box::new(Expression::NonTerminal("B".to_string())),
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

    if grammar.parse(&str) {
        println!("Matched!");
    } else {
        println!("Not matched");
    }
}
