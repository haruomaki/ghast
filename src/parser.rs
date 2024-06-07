use monapa::*;

pub use monapa::ParseError;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Ghast {
    Symbol(String),
    Fn(String, Box<Ghast>),
    Apply(Box<Ghast>, Box<Ghast>),
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

fn paren() -> Parser<Ghast> {
    (pdo! {
        single('(');
        whitespace() * ..;
        content <- paren();
        whitespace() * ..;
        single(')');
        return content
    }) | (ghast_fn() | ghast_symbol() | ghast_i32())
}

fn ghast_apply_left() -> Parser<Ghast> {
    paren()
}

fn ghast_apply_right() -> Parser<Option<Ghast>> {
    // FIXME: 余計なカッコを明示しないといけないバグを修正
    (pdo! {
        whitespace() * (1..);
        left <- ghast_apply_left();
        return Some(left)
    }) | Parser::ret(None)
}

pub fn ghast_master() -> Parser<Ghast> {
    pdo! {
        // Applyの左再帰を除去した
        left <- ghast_apply_left();
        right <- ghast_apply_right();
        return match right {
            Some(right) => Ghast::Apply(Box::new(left), Box::new(right)),
            None => left,
        }
    }
}
