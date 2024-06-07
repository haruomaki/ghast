use crate::ghast::{Binop, Ghast, Literal};

#[derive(Clone, Debug)]
pub enum CoreLang {
    Symbol(String),
    Fn(String, Box<CoreLang>),
    Apply(Box<CoreLang>, Box<CoreLang>),
    Lit(Literal),
}

fn split_at(binop: Binop, p: usize) -> (Binop, Binop) {
    let ops_size = binop.ops.len();
    let new_front = Binop {
        terms: binop.terms[0..=p].to_vec(),
        ops: binop.ops[0..p].to_vec(),
    };
    assert_eq!(ops_size + 1, binop.terms.len());
    let new_back = Binop {
        terms: binop.terms[p + 1..ops_size + 1].to_vec(),
        ops: binop.ops[p + 1..ops_size].to_vec(),
    };
    (new_front, new_back)
}

pub fn convert_into_core(ghast: Ghast) -> CoreLang {
    match ghast {
        Ghast::Symbol(name) => CoreLang::Symbol(name),
        Ghast::Fn(param, body) => CoreLang::Fn(param, Box::new(convert_into_core(*body))),
        Ghast::Lit(literal) => CoreLang::Lit(literal),
        Ghast::Binop(binop) => {
            // 優先度の低い順に、左結合なら右から、右結合なら左から探索していく。
            // 今のところ演算子は" "だけ。左結合なので右から探索。

            if binop.terms.len() == 1 {
                let term = binop.terms.into_iter().next().expect("termsは長さ1のはず"); // 先頭要素を所有権ごと取得
                return convert_into_core(term);
            }

            match binop.ops.iter().rposition(|op| op == " ") {
                Some(first_idx) => {
                    let (b, f) = split_at(binop, first_idx);
                    let bcore = convert_into_core(Ghast::Binop(b));
                    let fcore = convert_into_core(Ghast::Binop(f));
                    CoreLang::Apply(Box::new(bcore), Box::new(fcore))
                }
                None => panic!("演算子\" \"が見つかりません！"),
            }
        }
    }
}
