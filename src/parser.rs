use crate::monad::{Functor, Monad};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseError {
    WrongTerminal(char, char),
    MissingNonTerminal,
    ChoiceMismatch(Box<ParseError>, Box<ParseError>),
    IncompleteParse,
    IterationError,
}

pub type ParseResult<T> = Result<T, ParseError>;

// パーサ定義を表す構造体。parseの引数に指定して（メンバ関数として呼び出して）使う。
// NOTE: クロージャを保持しているためClone不可。
pub struct Parser<T> {
    _parse: Box<dyn Fn(&mut std::str::Chars) -> ParseResult<T>>,
}

// parse関数
impl<T> Parser<T> {
    pub fn parse(&self, input: impl AsRef<str>) -> ParseResult<T> {
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
    fn bind<S, F: Fn(T) -> Parser<S> + 'static>(self, f: F) -> Parser<S> {
        Parser {
            _parse: Box::new(move |iter| {
                let res = (self._parse)(iter)?;
                let par = f(res);
                (par._parse)(iter)
            }),
        }
    }

    // モナドのreturn関数
    // _parseが複数回呼び出される可能性がある（Fn）ためCloneを付ける。
    fn ret(value: T) -> Self
    where
        T: Clone,
    {
        Parser {
            _parse: Box::new(move |_| Ok(value.clone())),
        }
    }
}

// 特定の一文字をパースしてその文字を返すパーサ
impl Parser<char> {
    pub fn terminal(expected: char) -> Self {
        Parser {
            _parse: Box::new(move |iter| match iter.next() {
                Some(c) => match c == expected {
                    true => return Ok(c),
                    false => Err(ParseError::WrongTerminal(c, expected)),
                },
                None => Err(ParseError::IterationError),
            }),
        }
    }
}

// 選択を表すコンビネータ
impl<T: 'static> Parser<T> {
    pub fn choice(p1: Self, p2: Self) -> Self {
        Parser {
            // INFO: Errのときだけ処理を続行する「?」演算子があればもっと簡潔に書ける
            _parse: Box::new(move |iter| match (p1._parse)(&mut iter.clone()) {
                Ok(res) => Ok(res),
                Err(e1) => match (p2._parse)(iter) {
                    Ok(res) => Ok(res),
                    Err(e2) => Err(ParseError::ChoiceMismatch(Box::new(e1), Box::new(e2))),
                },
            }),
        }
    }
}
