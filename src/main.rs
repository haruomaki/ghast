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
}

type ParseResult<T> = Result<T, ParseError>;

struct Parser<T> {
    _parse: Box<dyn Fn(std::str::Chars) -> ParseResult<T>>,
}

impl<T: 'static> Parser<T> {
    fn parse(&self, input: impl AsRef<str>) -> ParseResult<T> {
        let iter = input.as_ref().chars();
        (self._parse)(iter)
    }

    // fn bind<S: 'static>(mut self, f: fn(T) -> Parser<S>) -> Parser<S> {
    //     Parser {
    //         _parse: Box::new(move |mut iter| {
    //             let res = (self._parse)(iter.clone())?;
    //             let mut par = f(res);
    //             (par._parse)(iter)
    //         }),
    //     }
    // }
}

impl Parser<()> {
    fn terminal(test: char) -> Self {
        Parser {
            _parse: Box::new(move |mut iter| {
                if let Some(c) = iter.next() {
                    if c == test {
                        return Ok(());
                    }
                }
                Err(ParseError::WrongTerminal)
            }),
        }
    }
}

fn main() {
    let input = "Hello, World!";

    let parser = Parser::terminal('H');
    let result = parser.parse(input);
    println!("{:?}", result);
}
