enum Expr {
    P,
    Cont(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
enum ParseError {
    WrongTerminal,
    MissingNonTerminal,
    IncompleteParse,
}

type ParseResult<T> = Result<T, ParseError>;

struct Parser<T, F>
where
    F: FnMut(std::str::Chars) -> ParseResult<T>,
{
    _parse: F,
}

impl<T, F: FnMut(std::str::Chars) -> ParseResult<T>> Parser<T, F> {
    fn parse(mut self, input: String) -> ParseResult<T> {
        (self._parse)(input.chars())
    }
}

fn terminal(test: char) -> Parser<(), impl FnMut(std::str::Chars) -> ParseResult<()>> {
    Parser {
        _parse: move |mut iter| {
            if let Some(c) = iter.next() {
                if c == test {
                    return Ok(());
                }
            }
            Err(ParseError::WrongTerminal)
        },
    }
}

fn main() {
    let input = "Hello, World!";

    let parser = terminal('H');
    let result = parser.parse(input.to_string());
    println!("{:?}", result);
}
