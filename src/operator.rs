// 演算子の結合方向と種類を定義するための列挙型
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Fixity {
    Prefix,
    Postfix,
    InfixLeft,
    InfixRight,
    CmpLeft,
    CmpRight,
    None,
}

pub const APPLY_NAME: &str = "APPLY";

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
        fixity: Fixity::InfixLeft,
        precedence: 10,
    },
    OperatorInfo {
        op: "-",
        name: "sub",
        fixity: Fixity::InfixLeft,
        precedence: 10,
    },
    OperatorInfo {
        op: "*",
        name: "mul",
        fixity: Fixity::InfixLeft,
        precedence: 20,
    },
    OperatorInfo {
        op: "/",
        name: "div",
        fixity: Fixity::InfixLeft,
        precedence: 20,
    },
    OperatorInfo {
        op: "+",
        name: "pos",
        fixity: Fixity::Prefix,
        precedence: 20,
    },
    OperatorInfo {
        op: "-",
        name: "neg",
        fixity: Fixity::Prefix,
        precedence: 20,
    },
    OperatorInfo {
        op: "!",
        name: "not",
        fixity: Fixity::Postfix,
        precedence: 15,
    },
    OperatorInfo {
        op: "",
        name: APPLY_NAME,
        fixity: Fixity::InfixLeft,
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

pub fn prefix_operators() -> Vec<&'static str> {
    OPERATORS
        .iter()
        .filter(|op_info| matches!(op_info.fixity, Fixity::Prefix))
        .map(|op_info| op_info.op)
        .collect()
}

pub fn postfix_operators() -> Vec<&'static str> {
    OPERATORS
        .iter()
        .filter(|op_info| matches!(op_info.fixity, Fixity::Postfix))
        .map(|op_info| op_info.op)
        .collect()
}

// 演算子をキーとしてその情報を取得する関数
pub fn info(op: &str) -> Option<&OperatorInfo> {
    // まず Infix を探す
    if let Some(info) = OPERATORS.iter().find(|op_info| {
        op_info.op == op
            && matches!(
                op_info.fixity,
                Fixity::InfixLeft | Fixity::InfixRight | Fixity::CmpLeft | Fixity::CmpRight
            )
    }) {
        Some(info)
    } else {
        // 次に Prefix
        OPERATORS
            .iter()
            .find(|op_info| op_info.op == op && op_info.fixity == Fixity::Prefix)
    }
}

pub fn info_unary(op: &str, is_prefix: bool) -> Option<&OperatorInfo> {
    let fixity = if is_prefix {
        Fixity::Prefix
    } else {
        Fixity::Postfix
    };
    OPERATORS
        .iter()
        .find(|op_info| op_info.op == op && op_info.fixity == fixity)
}
