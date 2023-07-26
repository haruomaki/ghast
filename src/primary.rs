use crate::{Expression, Grammar};

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
