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

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Ghast {
    Symbol(String),
    Fn(String, Box<Ghast>),
    Binop(Binop),
    Lit(Literal),
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
        op <- single('&');
        whitespace() * ..;
        return op.to_string()
    }) | pdo! {
        whitespace() * (1..);
        return ' '.to_string()
    }
}

fn ghast_binop_rest() -> Parser<Vec<(String, Ghast)>> {
    (pdo! {
        op <- binop();
        right <- term();
        return (op, right)
    } * ..)
        | Parser::ret(vec![])
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
    ghast_fn() | ghast_symbol() | ghast_lit() | paren()
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

fn ghast_lit() -> Parser<Ghast> {
    pdo! {
        // I32
        num <- literal_digit() * (1..);
        let num_str = num.iter().collect::<String>();
        return Ghast::Lit(Literal::I32(num_str.parse().unwrap()))
    }
}

pub fn ghast_master() -> Parser<Ghast> {
    ghast_binop()
}
