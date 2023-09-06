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
    _parse: Box<dyn FnMut(std::str::Chars) -> ParseResult<T>>,
}

impl<T> Parser<T> {
    fn parse(mut self, input: String) -> ParseResult<T> {
        (self._parse)(input.chars())
    }
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
    let result = parser.parse(input.to_string());
    println!("{:?}", result);
}
