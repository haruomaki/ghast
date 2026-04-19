use crate::operator::{available_operators_without_space, postfix_operators, prefix_operators};
use monapa::*;

pub use monapa::ParseError;

#[derive(Clone, Debug)]
pub struct Binop {
    pub terms: Vec<FlatIR>,
    pub ops: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum Literal {
    I32(i32),
}

#[derive(Clone, Debug)]
pub enum FlatIR {
    Symbol(String),
    Fn(String, Box<FlatIR>),
    UnaryOp(String, Box<FlatIR>),
    Binop(Binop),
    Lit(Literal),
    Tuple(Vec<FlatIR>),
}

/// 空白を消費するパーサ
fn ws() -> Parser<()> {
    (whitespace() * ..).void()
}

/// 変数名・仮引数名などを表すパーサ
fn id() -> Parser<String> {
    pdo! {
        start <- Parser::satisfy(|c| c == '_' || c.is_alphabetic());
        conti <- Parser::satisfy(|c| c == '_' || c.is_alphanumeric()) * ..;
        let idvec = vec![vec![start], conti].concat();
        return idvec.iter().collect()
    }
}

// ---------------------------
// 単項演算子
// ---------------------------

/// 後置単項演算子のパーサ
fn postfix_term() -> Parser<FlatIR> {
    // 左再帰を回避するため、演算子を任意個一気に消費した後、無理矢理組み立てる
    pdo! {
        base <- atom();
        ops <- (ws() >> choice(postfix_operators().into_iter().map(|op| chunk(op)))) * (..);
        return ops.into_iter().fold(base, |acc, op| {
            FlatIR::UnaryOp(op, Box::new(acc))
        })
    }
}

/// 単項演算子のパーサ
fn prefix_term() -> Parser<FlatIR> {
    pdo! {
        op <- choice(prefix_operators().into_iter().map(|op| chunk(op)));
        ws();
        term <- term();
        return FlatIR::UnaryOp(op, Box::new(term))
    }
}

// ---------------------------
// 二項演算子
// ---------------------------

/// 二項演算子自体を表すパーサ
fn binop_itself() -> Parser<String> {
    (pdo! {
        ws();
        op <- choice(available_operators_without_space().into_iter().map(|op| chunk(op)));
        ws();
        return op.to_string()
    }) | pdo! {
        whitespace() * (1..);
        return " ".to_string()
    }
}

/// １回以上の [op, term] の繰り返しをパースする
fn binop_rest() -> Parser<Vec<(String, FlatIR)>> {
    (pdo! {
        op <- binop_itself();
        t <- term();
        return (op, t)
    }) * (1..)
}

/// 二項演算子を右辺と左辺込みでパースする
fn binop() -> Parser<FlatIR> {
    pdo! {
        head <- term();
        rest <- binop_rest();
        return if rest.is_empty() {
            // TODO: 条件分岐を削除
            panic!("ぴえん");
        } else {
            let mut terms = vec![head];
            let mut ops = vec![];
            for (op, term) in rest {
                terms.push(term);
                ops.push(op);
            }
            FlatIR::Binop(Binop{terms, ops})
        }
    }
}

// ---------------------------
// 全般
// ---------------------------

/// 括弧に包まれた表現をパース
fn paren() -> Parser<FlatIR> {
    single('(') >> expr() << single(')')
}

/// シンボルを表すパーサ
fn symbol() -> Parser<FlatIR> {
    id().map(FlatIR::Symbol)
}

/// ラムダ式を表すパーサ
fn fun() -> Parser<FlatIR> {
    pdo! {
        single('\\');
        arg <- id();
        whitespace() * (..);
        chunk("->");
        whitespace() * (..);
        cont <- expr();
        return FlatIR::Fn(arg, Box::new(cont))
    }
}

/// リテラルを表すパーサ
fn literal() -> Parser<FlatIR> {
    pdo! {
        // I32
        num <- ascii_digit() * (1..); // TODO: 文字列、小数などにも対応
        let num_str = num.iter().collect::<String>();
        return FlatIR::Lit(Literal::I32(num_str.parse().unwrap()))
    }
}

/// タプル（丸括弧で囲まれたカンマ区切りの項）のパーサ
fn tuple() -> Parser<FlatIR> {
    pdo! {
        single('(');
        terms <- expr().sep_by(single(','));
        ws(); // 空の括弧内 or 末尾カンマ後の空白を許容
        single(')');
        return FlatIR::Tuple(terms)
    }
}

/// 後置演算子の前に配置できる表現
fn atom() -> Parser<FlatIR> {
    ws() >> (literal() | symbol() | paren() | tuple()) << ws()
}

/// 二項演算子の項になれる表現
pub fn term() -> Parser<FlatIR> {
    ws() >> (prefix_term() | postfix_term() | atom()) << ws()
}

/// 二項演算子の項になれないものも含む、あらゆる表現
pub fn expr() -> Parser<FlatIR> {
    ws() >> (binop() | fun() | term()) << ws()
}

pub fn ghast() -> Parser<FlatIR> {
    expr()
}

// =====================================
// テスト
// =====================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unary_minus_literal() {
        let ast = expr().parse("-5").unwrap();
        match ast {
            FlatIR::UnaryOp(op, arg) => {
                assert_eq!(op, "-");
                match *arg {
                    FlatIR::Lit(Literal::I32(value)) => assert_eq!(value, 5),
                    other => panic!("unexpected unary operand: {:?}", other),
                }
            }
            other => panic!("unexpected parse result: {:?}", other),
        }
    }

    #[test]
    fn parse_add_unary_minus() {
        let ast = expr().parse("5 + - 3").unwrap();
        match ast {
            FlatIR::Binop(binop) => {
                assert_eq!(binop.ops, vec!["+".to_string()]);
                match &binop.terms[1] {
                    FlatIR::UnaryOp(op, arg) => {
                        assert_eq!(op, "-");
                        match **arg {
                            FlatIR::Lit(Literal::I32(value)) => assert_eq!(value, 3),
                            ref other => panic!("unexpected unary operand: {:?}", other),
                        }
                    }
                    other => panic!("unexpected right term: {:?}", other),
                }
            }
            other => panic!("unexpected parse result: {:?}", other),
        }
    }
}
