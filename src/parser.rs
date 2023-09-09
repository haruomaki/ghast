use crate::monad::{Functor, Monad};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseError {
    WrongTerminal(char, char),
    MissingNonTerminal,
    ChoiceMismatch(Box<ParseError>, Box<ParseError>),
    SatisfyError,
    ManyError,
    IncompleteParse,
    IterationError,
}

pub type ParseResult<T> = Result<T, ParseError>;

// パーサ定義を表す構造体。parseの引数に指定して（メンバ関数として呼び出して）使う。
// NOTE: クロージャを保持しているためClone不可。
pub struct Parser<T> {
    _parse: Box<dyn FnOnce(&mut std::str::Chars) -> ParseResult<T>>,
}

// 内部だけで使う
fn new<T>(_parse: impl FnOnce(&mut std::str::Chars) -> ParseResult<T> + 'static) -> Parser<T> {
    Parser {
        _parse: Box::new(_parse),
    }
}

// parse関数
impl<T> Parser<T> {
    pub fn parse(self, input: impl AsRef<str>) -> ParseResult<T> {
        let mut iter = input.as_ref().chars();
        (self._parse)(&mut iter)
    }
}

// Functorトレイト
impl<T> Functor for Parser<T> {
    type A = T;
    type Lifted<B> = Parser<B>;
}

// Monadトレイト
impl<T: 'static> Monad for Parser<T> {
    // モナドのbind関数。連接を表す
    fn bind<S, F: FnOnce(T) -> Parser<S> + 'static>(self, f: F) -> Parser<S> {
        new(move |iter| {
            let res = (self._parse)(iter)?;
            let par = f(res);
            (par._parse)(iter)
        })
    }

    // モナドのreturn関数
    fn ret(value: T) -> Self {
        new(move |_| Ok(value))
    }
}

// 特定の一文字をパースしてその文字を返すパーサ
impl Parser<char> {
    pub fn terminal(expected: char) -> Self {
        new(move |iter| match iter.next() {
            Some(c) => match c == expected {
                true => return Ok(c),
                false => Err(ParseError::WrongTerminal(c, expected)),
            },
            None => Err(ParseError::IterationError),
        })
    }

    // TODO: boodでなくResultにして、エラーを詳細化
    pub fn satisfy(f: impl Fn(char) -> bool + 'static) -> Self {
        new(move |iter| match iter.next() {
            Some(c) => match f(c) {
                true => return Ok(c),
                false => Err(ParseError::SatisfyError),
            },
            None => Err(ParseError::IterationError),
        })
    }

    pub fn ascii_digit() -> Self {
        Parser::satisfy(|c| char::is_ascii_digit(&c))
    }
    pub fn digit(radix: u32) -> Self {
        Parser::satisfy(move |c| char::is_digit(c, radix))
    }
    pub fn numeric() -> Self {
        Parser::satisfy(char::is_numeric)
    }
}

// 選択を表すコンビネータ
impl<T: 'static> Parser<T> {
    pub fn choice(p1: Self, p2: Self) -> Self {
        // INFO: Errのときだけ処理を続行する「?」演算子があればもっと簡潔に書ける？（でもiter_backupは無理かも）
        new(move |iter| {
            let iter_backup = iter.clone();
            match (p1._parse)(iter) {
                Ok(res) => Ok(res),
                Err(e1) => {
                    *iter = iter_backup;
                    match (p2._parse)(iter) {
                        Ok(res) => Ok(res),
                        Err(e2) => Err(ParseError::ChoiceMismatch(Box::new(e1), Box::new(e2))),
                    }
                }
            }
        })
    }
}

// choiceと等価の演算子 <|>
impl<T: 'static> std::ops::BitOr for Parser<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Parser::choice(self, rhs)
    }
}

// 繰り返しを表すコンビネータ
// impl<T: 'static> Parser<T> {
//     pub fn many(self, min: Option<usize>, max: Option<usize>) -> Parser<Vec<T>> {
//         new(move |iter| {
//             let mut count = 1;
//             let mut asts = vec![];
//             while match max {
//                 Some(v) => count <= v,
//                 None => true,
//             } {
//                 let iter_backup = iter.clone();
//                 let res = (self._parse)(iter);
//                 if let Ok(ast) = res {
//                     asts.push(ast);
//                     count += 1;
//                 } else {
//                     *iter = iter_backup;
//                     break;
//                 }
//             }

//             if min.is_some() && asts.len() < min.unwrap() {
//                 Err(ParseError::ManyError)
//             } else {
//                 Ok(asts)
//             }
//         })
//     }
// }
