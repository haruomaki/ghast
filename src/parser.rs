use crate::monad::{Functor, Monad};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseError {
    WrongTerminal,
    MissingNonTerminal,
    IncompleteParse,
    IterationError,
}

pub type ParseResult<T> = Result<T, ParseError>;

// パーサ定義を表す構造体。parseの引数に指定して（メンバ関数として呼び出して）使う。
// NOTE: クロージャを保持しているためClone不可。
pub struct Parser<T> {
    _parse: Box<dyn Fn(&mut std::str::Chars) -> ParseResult<T>>,
}

impl<T> Parser<T> {
    pub fn parse(&self, input: impl AsRef<str>) -> ParseResult<T> {
        let mut iter = input.as_ref().chars();
        (self._parse)(&mut iter)
    }
}

impl<T> Functor for Parser<T> {
    type A = T;
    type Lifted<B> = Parser<B>;
}

impl<T: 'static> Monad for Parser<T> {
    // モナドのbind関数
    fn bind<S, F: Fn(T) -> Parser<S> + 'static>(self, f: F) -> Parser<S> {
        Parser {
            _parse: Box::new(move |mut iter| {
                let res = (self._parse)(&mut iter)?;
                let par = f(res);
                (par._parse)(&mut iter)
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

impl Parser<char> {
    pub fn terminal(test: char) -> Self {
        Parser {
            _parse: Box::new(move |iter| match iter.next() {
                Some(c) => match c == test {
                    true => return Ok(c),
                    false => Err(ParseError::WrongTerminal),
                },
                None => Err(ParseError::IterationError),
            }),
        }
    }
}
