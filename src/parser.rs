use monapa::*;

pub use monapa::ParseError;

#[derive(Clone, Debug)]
pub struct Binops {
    terms: Vec<Ghast>,
    ops: Vec<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Ghast {
    Symbol(String),
    Fn(String, Box<Ghast>),
    Binops(Binops),
    I32(i32),
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
    pdo! {
        whitespace() * ..;
        op <- single('&');
        whitespace() * ..;
        return op.to_string()
    }
}

fn append_binop_term(left: Ghast, op: String, mut right: Binops) -> Binops {
    right.terms.push(left);
    right.ops.push(op);
    right
}

fn binops_new(term: Ghast) -> Binops {
    Binops {
        terms: vec![term],
        ops: vec![],
    }
}

fn ghast_binop_right() -> Parser<Binops> {
    (pdo! {
        left <- term();
        op <- binop();
        right <- ghast_binop_right();
        return append_binop_term(left, op, right)
    }) | term().bind(|t| Parser::ret(binops_new(t)))
}

fn term() -> Parser<Ghast> {
    ghast_fn() | ghast_symbol() | ghast_i32()
}

fn ghast_symbol() -> Parser<Ghast> {
    id().bind(|id| Parser::ret(Ghast::Symbol(id)))
}

fn ghast_fn() -> Parser<Ghast> {
    pdo! {
        single('\\');
        arg <- id();
        whitespace() * (..);
        chunk("->");
        whitespace() * (..);
        cont <- ghast_master();
        return Ghast::Fn(arg, Box::new(cont))
    }
}

fn ghast_i32() -> Parser<Ghast> {
    pdo! {
        num <- literal_digit() * (1..);
        let num_str = num.iter().collect::<String>();
        return Ghast::I32(num_str.parse().unwrap())
    }
}

pub fn ghast_master() -> Parser<Ghast> {
    pdo! {
        bp <- ghast_binop_right();
        return Ghast::Binops(bp)
    }
}
