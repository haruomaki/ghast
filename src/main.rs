#[derive(Debug)]
enum Expression {
    Terminal(char),
    NonTerminal(String),
    Sequence(Box<Expression>, Box<Expression>),
    OrderedChoice(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn parse(&self, grammar: &Grammar, p: &mut Parser) -> bool {
        println!("parse: {:?}, i = {}", self, p.i);
        let i = p.i;
        let result = match self {
            Expression::Terminal(c) => {
                if p.str.chars().nth(p.i) == Some(*c) {
                    p.i += 1;
                    true
                } else {
                    false
                }
            }
            Expression::NonTerminal(sym) => grammar.parse_symbol(sym, p),
            Expression::Sequence(e1, e2) => e1.parse(grammar, p) && e2.parse(grammar, p),
            Expression::OrderedChoice(e1, e2) => e1.parse(grammar, p) || e2.parse(grammar, p),
        };

        if !result {
            p.i = i
        };

        println!("end: {:?}, i = {}, result = {}", self, p.i, result);
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

    fn parse(&self, p: &mut Parser) -> bool {
        self.parse_symbol(&self.start_symbol, p) && p.i == p.str.len()
    }

    fn parse_symbol(&self, symbol: &str, p: &mut Parser) -> bool {
        if let Some(expr) = self
            .rules
            .iter()
            .find(|(sym, _)| *sym == symbol)
            .map(|(_, e)| e)
        {
            expr.parse(self, p)
        } else {
            false
        }
    }
}

struct Parser {
    str: String,
    i: usize,
}

fn main() {
    let input = String::from("ab");
    let mut parser = Parser { str: input, i: 0 };

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
    grammar.add_rule("B", Expression::Terminal('b'));

    if grammar.parse(&mut parser) {
        println!("Matched!");
    } else {
        println!("Not matched");
    }
}
