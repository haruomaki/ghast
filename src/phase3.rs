use crate::operator::*;
use crate::phase2::{Binop, FlatIR, Literal};

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone, Debug)]
pub enum Ghast {
    Symbol(String),
    Fn(String, Box<Ghast>),
    Apply(Box<Ghast>, Box<Ghast>),
    Lit(Literal),
    Tuple(Vec<Ghast>),
    Block(Vec<Ghast>),
    If(Box<Ghast>, Box<Ghast>, Box<Ghast>), // condition, then, else
}

pub type Env = HashMap<String, Value>;

// ===========================
// Value列挙体
// ===========================

#[derive(Clone, Debug)]
pub enum Value {
    I32(i32),
    Bool(bool),
    Closure(String, Box<Ghast>, Env),
    Tuple(Vec<Value>),
    Builtin(&'static str),
    Unit,
}

impl Value {
    fn as_i32(&self) -> i32 {
        match self {
            Value::I32(value) => *value,
            _ => panic!("整数値ではありません: {:?}", self),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::I32(value) => write!(f, "{}", value),
            Value::Bool(true) => write!(f, "true"),
            Value::Bool(false) => write!(f, "false"),
            Value::Closure(param, body, _env) => {
                write!(f, "closure({:?}, {:?})", param, body)
            }
            Value::Tuple(elements) => {
                write!(
                    f,
                    "({})",
                    elements
                        .iter()
                        .map(|e| format!("{}", e))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Value::Builtin(name) => write!(f, "{}", name),
            Value::Unit => write!(f, "unit"),
        }
    }
}

// ===========================
// ASTの評価
// ===========================

pub fn eval(ast: &Ghast) -> Value {
    let mut env = Env::new();
    eval_with_env(ast, &mut env)
}

fn eval_with_env(ast: &Ghast, env: &mut Env) -> Value {
    match ast {
        Ghast::Symbol(name) => match env.get(name) {
            Some(value) => value.clone(),
            None => match name.as_str() {
                "add" => Value::Builtin("add"),
                "sub" => Value::Builtin("sub"),
                "mul" => Value::Builtin("mul"),
                "div" => Value::Builtin("div"),
                "neg" => Value::Builtin("neg"),
                "pos" => Value::Builtin("pos"),
                "not" => Value::Builtin("not"),
                "print" => Value::Builtin("print"),
                _ => panic!("未定義のシンボルです: {}", name),
            },
        },
        Ghast::Fn(param, body) => Value::Closure(param.clone(), body.clone(), env.clone()),
        Ghast::Apply(func, args) => {
            let func_value = eval_with_env(func, env);
            let args_value = eval_with_env(args, env);

            match func_value {
                Value::Closure(param, body, closure_env) => {
                    let arg = match args_value {
                        Value::Tuple(mut elements) => {
                            if elements.len() == 1 {
                                elements.remove(0)
                            } else {
                                Value::Tuple(elements)
                            }
                        }
                        other => other,
                    };
                    let mut new_env = closure_env.clone();
                    new_env.insert(param, arg);
                    eval_with_env(&body, &mut new_env)
                }
                Value::Builtin(name) => match name {
                    "add" => match args_value {
                        Value::Tuple(mut elements) if elements.len() == 2 => {
                            let rhs = elements.pop().unwrap().as_i32();
                            let lhs = elements.pop().unwrap().as_i32();
                            Value::I32(lhs + rhs)
                        }
                        _ => panic!("add は 2 つの整数を取ります"),
                    },
                    "sub" => match args_value {
                        Value::Tuple(mut elements) if elements.len() == 2 => {
                            let rhs = elements.pop().unwrap().as_i32();
                            let lhs = elements.pop().unwrap().as_i32();
                            Value::I32(lhs - rhs)
                        }
                        _ => panic!("sub は 2 つの整数を取ります"),
                    },
                    "mul" => match args_value {
                        Value::Tuple(mut elements) if elements.len() == 2 => {
                            let rhs = elements.pop().unwrap().as_i32();
                            let lhs = elements.pop().unwrap().as_i32();
                            Value::I32(lhs * rhs)
                        }
                        _ => panic!("mul は 2 つの整数を取ります"),
                    },
                    "div" => match args_value {
                        Value::Tuple(mut elements) if elements.len() == 2 => {
                            let rhs = elements.pop().unwrap().as_i32();
                            let lhs = elements.pop().unwrap().as_i32();
                            Value::I32(lhs / rhs)
                        }
                        _ => panic!("div は 2 つの整数を取ります"),
                    },
                    "neg" => match args_value {
                        Value::I32(value) => Value::I32(-value),
                        _ => panic!("neg は 1 つの整数を取ります"),
                    },
                    "pos" => match args_value {
                        Value::I32(value) => Value::I32(value),
                        _ => panic!("pos は 1 つの整数を取ります"),
                    },
                    "not" => match args_value {
                        Value::I32(value) => Value::I32(if value != 0 { 0 } else { 1 }),
                        _ => panic!("not は 1 つの整数を取ります"),
                    },
                    "print" => {
                        // TODO: 関数には必ずタプルが渡されることにしているが、ちょっと処理がめんどい、、
                        match args_value {
                            Value::Tuple(vals) => {
                                // 空白区切りで出力
                                println!(
                                    "{}",
                                    vals.iter()
                                        .map(|v| format!("{}", v))
                                        .collect::<Vec<_>>()
                                        .join(" ")
                                );
                            }
                            _ => panic!("関数への入力がタプルでありません"),
                        }
                        Value::Unit
                    }
                    _ => panic!("未知の組み込み関数です: {}", name),
                },
                other => panic!("適用可能な関数ではありません: {:?}", other),
            }
        }
        Ghast::Block(exprs) => {
            let mut result = Value::Unit;
            for expr in exprs {
                result = eval_with_env(expr, env);
            }
            result
        }
        Ghast::If(cond, then, else_expr) => {
            let cond_value = eval_with_env(cond, env);
            match cond_value {
                Value::Bool(true) => eval_with_env(then, env),
                Value::Bool(false) => eval_with_env(else_expr, env),
                other => panic!("条件式はブール値である必要があります: {:?}", other),
            }
        }
        Ghast::Lit(literal) => match literal {
            Literal::I32(value) => Value::I32(*value),
            Literal::Bool(value) => Value::Bool(*value),
            Literal::Str(_value) => Value::Builtin("str"), // Placeholder for string value
        },
        Ghast::Tuple(elements) => Value::Tuple(
            elements
                .iter()
                .map(|element| eval_with_env(element, env))
                .collect(),
        ),
    }
}

// ===========================
// ユーティリティ関数
// ===========================

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
        .map(|op| info(op).unwrap().precedence)
        .min()
        .expect("opsは空でない必要があります");

    // 最小優先度を持つ演算子のインデックスと結合性を収集
    let min_ops: Vec<_> = ops
        .iter()
        .enumerate()
        .filter(|(_, op)| info(op).unwrap().precedence == min_prec)
        .map(|(i, op)| (i, info(op).unwrap().fixity))
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
            Fixity::InfixLeft => Ok(min_ops.iter().map(|(i, _)| i).max().unwrap().to_owned()),
            Fixity::InfixRight => Ok(min_ops.iter().map(|(i, _)| i).min().unwrap().to_owned()),
            _ => panic!("CmpLeftとCmpRightは未実装です"),
        }
    }
}

// 一変数関数への引数を、要素数1のタプルへ変換する
fn ensure_tuple(arg: Ghast) -> Ghast {
    match arg {
        Ghast::Tuple(_) => arg, // 既にタプルなら何もしない
        _ => Ghast::Tuple(vec![arg]),
    }
}

pub fn convert_into_ghast(binop_ir: FlatIR) -> Ghast {
    match binop_ir {
        FlatIR::Symbol(name) => Ghast::Symbol(name),
        FlatIR::Fn(param, body) => Ghast::Fn(param, Box::new(convert_into_ghast(*body))),
        FlatIR::Lit(literal) => Ghast::Lit(literal),
        FlatIR::Tuple(elems) => Ghast::Tuple(elems.into_iter().map(convert_into_ghast).collect()),
        FlatIR::Block(exprs) => Ghast::Block(exprs.into_iter().map(convert_into_ghast).collect()),
        FlatIR::IfElse(cond, then, else_expr) => Ghast::If(
            Box::new(convert_into_ghast(*cond)),
            Box::new(convert_into_ghast(*then)),
            Box::new(convert_into_ghast(*else_expr)),
        ),
        FlatIR::UnaryOp(op, arg) => {
            let name = if let Some(info) = info_unary(&op, true) {
                info.name
            } else if let Some(info) = info_unary(&op, false) {
                info.name
            } else {
                panic!("未知の単項演算子です: {}", op);
            };
            Ghast::Apply(
                Box::new(Ghast::Symbol(name.to_string())),
                Box::new(convert_into_ghast(*arg)),
            )
        }
        FlatIR::Binop(binop) => {
            // 優先度の低い順に、左結合なら右から、右結合なら左から探索していく。
            // 今のところ演算子は" "だけ。左結合なので右から探索。

            if binop.terms.len() == 1 {
                let term = binop.terms.into_iter().next().expect("termsは長さ1のはず"); // 先頭要素を所有権ごと取得
                return convert_into_ghast(term);
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
                        let bcore = convert_into_ghast(FlatIR::Binop(b));
                        let fcore = convert_into_ghast(FlatIR::Binop(f));
                        (bcore, fcore)
                    } else {
                        let name = info(&binop.ops[pivot]).unwrap().name;
                        if name == "__block" {
                            let (b, f) = split_at(binop, pivot);
                            let bcore = convert_into_ghast(FlatIR::Binop(b));
                            let fcore = convert_into_ghast(FlatIR::Binop(f));
                            return Ghast::Block(vec![bcore, fcore]);
                        } else {
                            let ncore = Ghast::Symbol(name.to_string());

                            let (b, f) = split_at(binop, pivot);
                            let bcore = convert_into_ghast(FlatIR::Binop(b));
                            let fcore = convert_into_ghast(FlatIR::Binop(f));
                            let args = Ghast::Tuple(vec![bcore, fcore]);
                            (ncore, args)
                        }
                    };

                    Ghast::Apply(Box::new(function), Box::new(ensure_tuple(arguments)))
                }
                Err(e) => panic!("find_min_precedence_index()でエラー: {:?}", e),
            }
        }
    }
}
