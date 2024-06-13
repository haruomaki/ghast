use crate::ghast::{Binop, Ghast, Literal};
use crate::operator::*;

use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum CoreValue {
    Symbol(String),
    Fn(String, Box<Core>),
    Apply(Box<Core>, Box<Core>),
    Lit(Literal),
    Tuple(Vec<Core>),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CoreType {
    Unknown,
    I32,
    Fn(Box<CoreType>, Box<CoreType>),
    Tuple(Vec<CoreType>),
}

pub type Core = (CoreValue, CoreType);

/// Binopを位置pで分割する
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

/// 最も優先度が小さい演算子を探索し、そのインデックスを返す
fn find_min_precedence_index(ops: &[String]) -> Result<usize, String> {
    // 最小優先度を見つける
    let min_prec = ops
        .iter()
        .map(|op| info(op).precedence)
        .min()
        .expect("opsは空でない必要があります");

    // 最小優先度を持つ演算子のインデックスと結合性を収集
    let min_ops: Vec<_> = ops
        .iter()
        .enumerate()
        .filter(|(_, op)| info(op).precedence == min_prec)
        .map(|(i, op)| (i, info(op).fixity))
        .collect();

    if min_ops.len() == 1 {
        // 並列がなければそのまま決定
        Ok(min_ops[0].0)
    } else {
        // 結合性の一貫性をチェック
        let fixities: HashSet<_> = min_ops.iter().map(|(_, fixity)| fixity).collect();
        let common_fixity = match fixities.len() {
            1 => *fixities.iter().next().unwrap(),
            _ => return Err("同じ優先度で異なる結合性を持つ演算子が混在しています".to_string()),
        };

        match common_fixity {
            Fixity::None => Err("非結合性(Fixity=None)の演算子を並列させています".to_string()),
            Fixity::Left => Ok(min_ops.iter().map(|(i, _)| i).max().unwrap().to_owned()),
            Fixity::Right => Ok(min_ops.iter().map(|(i, _)| i).min().unwrap().to_owned()),
            _ => panic!("CmpLeftとCmpRightは未実装です"),
        }
    }
}

// 一変数関数への引数を、要素数1のタプルへ変換する
fn ensure_tuple(arg: Core) -> Core {
    match arg.clone() {
        (CoreValue::Tuple(_), _) => arg, // 既にタプルなら何もしない
        (_, ty) => (CoreValue::Tuple(vec![arg]), CoreType::Tuple(vec![ty])),
    }
}

pub fn convert_into_core(ghast: Ghast) -> Core {
    match ghast {
        Ghast::Symbol(name) => (CoreValue::Symbol(name), CoreType::Unknown),
        Ghast::Fn(param, body) => (
            CoreValue::Fn(param, Box::new(convert_into_core(*body))),
            CoreType::Unknown,
        ),
        Ghast::Lit(literal) => (CoreValue::Lit(literal), CoreType::Unknown),
        Ghast::Tuple(elems) => (
            CoreValue::Tuple(elems.into_iter().map(|e| convert_into_core(e)).collect()),
            CoreType::Unknown,
        ),
        Ghast::Binop(binop) => {
            // 優先度の低い順に、左結合なら右から、右結合なら左から探索していく。
            // 今のところ演算子は" "だけ。左結合なので右から探索。

            if binop.terms.len() == 1 {
                let term = binop.terms.into_iter().next().expect("termsは長さ1のはず"); // 先頭要素を所有権ごと取得
                return convert_into_core(term);
            }

            let pivot = find_min_precedence_index(&binop.ops);
            match pivot {
                Ok(pivot) => {
                    // "APPLY"なら関数適用、それ以外なら関数名を取得し適用
                    let apply_info = OPERATORS
                        .iter()
                        .find(|opinfo| opinfo.name == APPLY_NAME)
                        .unwrap();
                    let (function, arguments) = if binop.ops[pivot] == apply_info.op {
                        let (b, f) = split_at(binop, pivot);
                        let bcore = convert_into_core(Ghast::Binop(b));
                        let fcore = convert_into_core(Ghast::Binop(f));
                        (bcore, fcore)
                    } else {
                        let name = info(&binop.ops[pivot]).name;
                        let ncore = CoreValue::Symbol(name.to_string());

                        let (b, f) = split_at(binop, pivot);
                        let bcore = convert_into_core(Ghast::Binop(b));
                        let fcore = convert_into_core(Ghast::Binop(f));
                        let args = CoreValue::Tuple(vec![bcore, fcore]);
                        ((ncore, CoreType::Unknown), (args, CoreType::Unknown))
                    };

                    (
                        CoreValue::Apply(Box::new(function), Box::new(ensure_tuple(arguments))),
                        CoreType::Unknown,
                    )
                }
                Err(e) => panic!("find_min_precedence_index()でエラー: {:?}", e),
            }
        }
    }
}
