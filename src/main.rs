#[allow(dead_code)]
enum Expr {
    P,
    Cont(Box<Expr>, Box<Expr>),
}

#[allow(dead_code)]
#[derive(Debug)]
enum ParseError {
    WrongTerminal,
    MissingNonTerminal,
    IncompleteParse,
    IterationError,
}

type ParseResult<T> = Result<T, ParseError>;

// パーサ定義を表す構造体。parseの引数に指定して（メンバ関数として呼び出して）使う。
// NOTE: クロージャを保持しているためClone不可。
struct Parser<T> {
    _parse: Box<dyn Fn(&mut std::str::Chars) -> ParseResult<T>>,
}

impl<T> Parser<T> {
    fn parse(&self, input: impl AsRef<str>) -> ParseResult<T> {
        let mut iter = input.as_ref().chars();
        (self._parse)(&mut iter)
    }
}

// モナドのbind関数
impl<T: 'static> Parser<T> {
    fn bind<S: 'static>(self, f: fn(T) -> Parser<S>) -> Parser<S> {
        Parser {
            _parse: Box::new(move |mut iter| {
                let res = (self._parse)(&mut iter)?;
                let par = f(res);
                (par._parse)(&mut iter)
            }),
        }
    }
}

// モナドのreturn関数
// _parseが複数回呼び出される可能性がある（Fn）ためCloneを付ける。
impl<S: 'static + Clone> Parser<S> {
    fn ret(value: S) -> Self {
        Parser {
            _parse: Box::new(move |_| Ok(value.clone())),
        }
    }
}

impl Parser<()> {
    fn terminal(test: char) -> Self {
        Parser {
            _parse: Box::new(move |iter| match iter.next() {
                Some(c) => match c == test {
                    true => return Ok(()),
                    false => Err(ParseError::WrongTerminal),
                },
                None => Err(ParseError::IterationError),
            }),
        }
    }
}

fn main() {
    let input = "Hello, World!";

    let parser = Parser::terminal('H');
    let parser = parser.bind(|()| Parser::terminal('e'));
    let parser = parser.bind(|()| Parser::terminal('l'));
    let parser = parser.bind(|()| Parser::terminal('l'));
    let parser = parser.bind(|()| {
        Parser::terminal('o');
        Parser::ret(455)
    });
    let result = parser.parse(input);
    println!("{:?}", result);
}
