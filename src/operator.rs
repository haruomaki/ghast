// 演算子の結合方向と比較演算子であるかどうかを定義するための列挙型
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Fixity {
    Left,
    Right,
    CmpLeft,
    CmpRight,
    None,
}

// 演算子の情報を保持する構造体
#[derive(Debug)]
pub struct OperatorInfo {
    pub op: &'static str,
    pub name: &'static str,
    pub fixity: Fixity,
    pub precedence: i16,
}

// グローバル定数として演算子とその情報を定義
pub const OPERATORS: &[OperatorInfo] = &[
    OperatorInfo {
        op: "+",
        name: "add",
        fixity: Fixity::Left,
        precedence: 10,
    },
    OperatorInfo {
        op: "-",
        name: "sub",
        fixity: Fixity::Left,
        precedence: 10,
    },
    OperatorInfo {
        op: " ",
        name: "APPLY",
        fixity: Fixity::Left,
        precedence: 300,
    },
];

// 演算子の一覧を返す関数
pub fn available_operators() -> Vec<&'static str> {
    OPERATORS.iter().map(|op_info| op_info.op).collect()
}

pub fn available_operators_without_space() -> Vec<&'static str> {
    available_operators()
        .into_iter()
        .filter(|&op| op != " ")
        .collect()
}

// 演算子をキーとしてその情報を取得する関数
pub fn info(op: &str) -> &OperatorInfo {
    OPERATORS
        .iter()
        .find(|&op_info| op_info.op == op)
        .expect("未知の演算子です")
}
