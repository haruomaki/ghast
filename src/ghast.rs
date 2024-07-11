use crate::operator::available_operators_without_space;
use monapa::*;

pub use monapa::ParseError;

#[derive(Clone, Debug)]
pub struct Binop {
    pub terms: Vec<Ghast>,
    pub ops: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum Literal {
    I32(i32),
}

#[derive(Clone, Debug)]
pub enum Ghast {
    Symbol(String),
    Fn(String, Box<Ghast>),
    Binop(Binop),
    Lit(Literal),
    Tuple(Vec<Ghast>),
}

fn id_start() -> Parser<char> {
    Parser::satisfy(|c| c == '_' || c.is_alphabetic())
}

fn id_continue() -> Parser<char> {
    Parser::satisfy(|c| c == '_' || c.is_alphanumeric())
}

fn id() -> Parser<String> {
    pdo! {
        start <- id_start();
        conti <- id_continue() * ..;
        let idvec = vec![vec![start], conti].concat();
        return idvec.iter().collect()
    }
}

fn literal_digit() -> Parser<char> {
    Parser::satisfy(|c| c.is_ascii_digit())
}

fn binop() -> Parser<String> {
    (pdo! {
        whitespace() * ..;
        op <- choice(available_operators_without_space().into_iter().map(|op| chunk(op)));
        whitespace() * ..;
        return op.to_string()
    }) | pdo! {
        whitespace() * (1..);
        return " ".to_string()
    }
}

fn ghast_binop_rest() -> Parser<Vec<(String, Ghast)>> {
    (pdo! {
        op <- binop();
        right <- term();
        return (op, right)
    }) * ..
}

fn ghast_binop() -> Parser<Ghast> {
    pdo! {
        head <- term();
        rest <- ghast_binop_rest();
        return if rest.is_empty() {
            head
        } else {
            let mut terms = vec![head];
            let mut ops = vec![];
            for (op, term) in rest {
                terms.push(term);
                ops.push(op);
            }
            Ghast::Binop(Binop{terms:terms, ops:ops})
        }
    }
}

fn paren() -> Parser<Ghast> {
    pdo! {
        single('(');
        whitespace() * ..;
        t <- ghast_master();
        whitespace() * ..;
        single(')');
        return t
    }
}

fn term() -> Parser<Ghast> {
    // (foo) is a value, (foo,) is a tuple
    ghast_fn() | ghast_symbol() | ghast_lit() | paren() | ghast_tuple()
}

fn ghast_symbol() -> Parser<Ghast> {
    id().bind(|id| Parser::ret(Ghast::Symbol(id)))
}

fn ghast_fn() -> Parser<Ghast> {
    pdo! {
        single('\\');
        arg <- id() | Parser::ret("".to_string());
        whitespace() * (..);
        chunk("->");
        whitespace() * (..);
        cont <- ghast_master();
        return Ghast::Fn(arg, Box::new(cont))
    }
}

fn ghast_lit() -> Parser<Ghast> {
    pdo! {
        // I32
        num <- literal_digit() * (1..);
        let num_str = num.iter().collect::<String>();
        return Ghast::Lit(Literal::I32(num_str.parse().unwrap()))
    }
}

fn tuple_tail() -> Parser<Option<Ghast>> {
    ghast_master().map(|g| Some(g)).choice(Parser::ret(None))
}

fn ghast_tuple() -> Parser<Ghast> {
    pdo! {
        single('(');
        gs <- (pdo! {
            g <- ghast_master();
            single(',');
            return g
        }) * ..;
        tail <- tuple_tail();
        single(')');
        return Ghast::Tuple(match tail{
            Some(g) => [gs, vec![g]].concat(),
            _ => gs,
        })
    }
}

pub fn ghast_master() -> Parser<Ghast> {
    pdo! {
        whitespace() * ..;
        binop <- ghast_binop();
        whitespace() * ..;
        return binop
    }
}
