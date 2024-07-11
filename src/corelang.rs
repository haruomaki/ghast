use crate::ghast::{Binop, Ghast, Literal};
use crate::operator::*;
use crate::sig;
use crate::utils::SemanticError;

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
#[derive(Clone, Debug, PartialEq, Eq)]
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
        Ghast::Symbol(name) => {
            let ty = match name.as_str() {
                "print" => sig! {I32 -> ()},
                "add" => sig! {(I32, I32) -> I32},
                _ => CoreType::Unknown,
            };
            (CoreValue::Symbol(name), ty)
        }
        Ghast::Fn(param, body) => {
            let (val, ty) = convert_into_core(*body);
            let body_core = (val, ty.clone());
            let fn_type = if param == "" {
                CoreType::Fn(Box::new(CoreType::Tuple(vec![])), Box::new(ty))
            } else {
                // TODO: 仮引数の使われ方から、仮引数の型を推論したい
                CoreType::Fn(Box::new(CoreType::Unknown), Box::new(ty))
            };
            (CoreValue::Fn(param, Box::new(body_core)), fn_type)
        }
        Ghast::Lit(literal) => {
            let ty = literal.get_type();
            (CoreValue::Lit(literal), ty)
        }
        Ghast::Tuple(elems) => {
            let vals: Vec<Core> = elems.into_iter().map(|e| convert_into_core(e)).collect();
            let tys = vals.iter().map(|v| v.1.clone()).collect();
            (CoreValue::Tuple(vals), CoreType::Tuple(tys))
        }
        Ghast::Binop(binop) => {
            // 優先度の低い順に、左結合なら右から、右結合なら左から探索していく。

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
                        let (front, back) = split_at(binop, pivot);
                        let fcore = convert_into_core(Ghast::Binop(front));
                        let bcore = convert_into_core(Ghast::Binop(back));
                        (fcore, bcore)
                    } else {
                        let name = info(&binop.ops[pivot]).name;
                        let ncore = CoreValue::Symbol(name.to_string());

                        let (front, back) = split_at(binop, pivot);
                        let fcore = convert_into_core(Ghast::Binop(front));
                        let bcore = convert_into_core(Ghast::Binop(back));
                        let atype = CoreType::Tuple(vec![fcore.1.clone(), bcore.1.clone()]);
                        let args = CoreValue::Tuple(vec![fcore, bcore]);
                        ((ncore, CoreType::Unknown), (args, atype))
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

impl Literal {
    fn get_type(&self) -> CoreType {
        match self {
            Literal::I32(_) => CoreType::I32,
        }
    }
}

pub fn type_inference((core_value, core_type): Core) -> Result<Core, SemanticError> {
    eprintln!("型推論です。{:?} | {:?}", core_value, core_type);

    match core_value.clone() {
        CoreValue::Fn(param, body) => {
            if let CoreType::Fn(param_type, _old_ret_type) = core_type {
                eprintln!(
                "Fnの型を推論します！ param:{} | body: {:?} が、その前に、bodyの型を確定させます。",
                param, body
            );
                let body = type_inference(*body)?;
                let ret_type = body.1.clone();
                eprintln!("Fnの戻り値の型は{:?}になります", ret_type);
                Ok((
                    CoreValue::Fn(param, Box::new(body)),
                    CoreType::Fn(param_type, Box::new(ret_type)),
                ))
            } else {
                panic!("Fnの型がFnでありません");
            }
        }

        CoreValue::Apply(func, arg) => {
            eprintln!("Applyの型を推論します！ func:{:?} | arg: {:?}", func, arg);
            if let CoreType::Fn(_, ret_type) = (*func).1 {
                if core_type == CoreType::Unknown {
                    eprintln!("Unknownだったものを{:?}に推論しました", ret_type);
                    Ok((core_value, *ret_type))
                } else {
                    eprintln!("何もしませんでした");
                    Ok((core_value, core_type))
                }
            } else {
                // Err(SemanticError::Misc)
                panic!("Applyの左辺が関数型でありません")
            }
        }
        _ => Ok((core_value, core_type)),
    }
}
