use crate::{Expression, Grammar};

fn any() -> Expression {
    Expression::Any
}

fn terminal(c: char) -> Expression {
    Expression::Terminal(c)
}

fn non_terminal(sym: &str) -> Expression {
    Expression::NonTerminal(sym.to_string())
}

fn sequence(e1: Expression, e2: Expression) -> Expression {
    Expression::Sequence(Box::new(e1), Box::new(e2))
}

fn ordered_choice(e1: Expression, e2: Expression) -> Expression {
    Expression::OrderedChoice(Box::new(e1), Box::new(e2))
}

pub fn create_peg_grammar() -> Grammar {
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

    grammar
}

pub fn grammar_test1() -> Grammar {
    let mut grammar = Grammar::new("S");
    grammar.add_rule(
        "S",
        ordered_choice(
            sequence(terminal('a'), sequence(non_terminal("S"), any())),
            terminal('b'),
        ),
    );

    grammar
}
