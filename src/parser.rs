#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseError {
    WrongSingle(char, char),
    WrongChunk(String, String),
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
    _parse: Box<dyn Fn(&mut std::str::Chars) -> ParseResult<T>>,
}

// 内部だけで使う
fn new<T>(_parse: impl Fn(&mut std::str::Chars) -> ParseResult<T> + 'static) -> Parser<T> {
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

// モナドとしての機能
impl<T: Clone + 'static> Parser<T> {
    // モナドのbind関数。連接を表す
    pub fn bind<S, F: Fn(T) -> Parser<S> + 'static>(self, f: F) -> Parser<S> {
        new(move |iter| {
            let res = (self._parse)(iter)?;
            let par = f(res);
            (par._parse)(iter)
        })
    }

    // モナドのreturn関数
    pub fn ret(value: T) -> Self {
        new(move |_| Ok(value.clone()))
    }
}

// 特定の一文字をパースしてその文字を返すパーサ
impl Parser<char> {
    pub fn single(expected: char) -> Self {
        new(move |iter| match iter.next() {
            Some(c) => match c == expected {
                true => return Ok(c),
                false => Err(ParseError::WrongSingle(c, expected)),
            },
            None => Err(ParseError::IterationError),
        })
    }

    pub fn chunk(expected: impl AsRef<str> + 'static) -> Parser<()> {
        new(move |iter| {
            let mut found = vec![];
            for ex in expected.as_ref().chars() {
                if let Some(c) = iter.next() {
                    found.push(c);
                    if c != ex {
                        return Err(ParseError::WrongChunk(
                            found.iter().collect(),
                            expected.as_ref().to_string(),
                        ));
                    }
                } else {
                    return Err(ParseError::IterationError);
                }
            }
            Ok(())
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

// 繰り返しを表すコンビネータ
impl<T: 'static> Parser<T> {
    pub fn many(self, min: Option<usize>, max: Option<usize>) -> Parser<Vec<T>> {
        new(move |iter| {
            let mut count = 1;
            let mut asts = vec![];
            while match max {
                Some(v) => count <= v,
                None => true,
            } {
                let iter_backup = iter.clone();
                let res = (self._parse)(iter);
                if let Ok(ast) = res {
                    asts.push(ast);
                    count += 1;
                } else {
                    *iter = iter_backup;
                    break;
                }
            }

            if min.is_some() && asts.len() < min.unwrap() {
                Err(ParseError::ManyError)
            } else {
                Ok(asts)
            }
        })
    }
}

impl<T: Clone + 'static> Parser<Vec<T>> {
    pub fn concat(self, rhs: Self) -> Self {
        new(move |iter| {
            let ast_left = (self._parse)(iter)?;
            let ast_right = (rhs._parse)(iter)?;
            Ok(vec![ast_left, ast_right].concat())
        })
    }
}

impl<T: Clone + 'static> Parser<T> {
    pub fn and<U: Clone + 'static, S: Clone + 'static>(self, rhs: Parser<S>) -> Parser<Vec<U>>
    where
        Parser<T>: Into<Parser<Vec<U>>>,
        Parser<S>: Into<Parser<Vec<U>>>,
    {
        let lhs: Parser<Vec<U>> = self.into();
        let rhs: Parser<Vec<U>> = rhs.into();
        lhs.concat(rhs)
    }
}

impl<T: 'static> Into<Parser<Vec<T>>> for Parser<T> {
    fn into(self) -> Parser<Vec<T>> {
        new(move |iter| {
            let ast = (self._parse)(iter)?;
            Ok(vec![ast])
        })
    }
}
