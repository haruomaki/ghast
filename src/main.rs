#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Expr {
    Hello,
    World,
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

pub trait Functor {
    type A;
    type Lifted<B>: Functor;

    // fn map<F, B>(self, f: F) -> Self::Lifted<B>
    // where
    //     F: FnMut(Self::A) -> B;
}

pub trait Pointed: Functor {
    // fn pure(t: Self::A) -> Self::Lifted<Self::A>;
}

pub trait Applicative: Pointed {
    // fn apply<F, B, C>(self, b: Self::Lifted<B>, f: F) -> Self::Lifted<C>
    // where
    //     F: FnMut(Self::A, B) -> C;
}

pub trait Monad: Functor {
    fn bind<B, F>(self, f: F) -> Self::Lifted<B>
    where
        F: Fn(Self::A) -> Self::Lifted<B> + 'static;

    fn ret(a: Self::A) -> Self
    where
        Self::A: Clone;
}

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

impl<T> Functor for Parser<T> {
    type A = T;
    type Lifted<B> = Parser<B>;
}

// モナドのbind関数
impl<T: 'static> Monad for Parser<T> {
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
        small_l1 <- Parser::terminal('l');
        small_l2 <- Parser::terminal('l');
        small_o <- Parser::terminal('o');
        Monad::ret(Expr::Hello)
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
    ($e:expr) => {
        $e
    };
}
