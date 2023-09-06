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
    fn bind<S: 'static>(self, f: impl Fn(T) -> Parser<S> + 'static) -> Parser<S> {
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
impl<S: Clone + 'static> Parser<S> {
    fn ret(value: S) -> Self {
        Parser {
            _parse: Box::new(move |_| Ok(value.clone())),
        }
    }
}

impl Parser<char> {
    fn terminal(test: char) -> Self {
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

fn main() {
    let input = "Hello, World!";

    // let parser = Parser::terminal('H').bind(move |large_h| {
    //     let pa = Parser::terminal('e').bind(move |small_e| Parser::ret(vec![large_h, small_e]))
    // });
    // 上記と等価
    let parser = mdo! {
        large_h <- Parser::terminal('H');
        small_e <- Parser::terminal('e');
        ret vec![large_h, small_e]
    };
    let result = parser.parse(input);
    println!("{:?}", result);
}

// https://blog-dry.com/entry/2020/12/25/130250#do-記法
#[macro_export]
macro_rules! mdo {
    ($i:ident <- $e:expr; $($t:tt)*) => {
        $e.bind(move |$i| mdo!($($t)*))
    };
    ($e:expr; $($t:tt)*) => {
        $e.bind(move |()| mdo!($($t)*))
    };
    (ret $e:expr) => {
        Parser::ret($e)
    };
}
