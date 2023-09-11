use crate::parser::Parser;

pub fn single(expected: char) -> Parser<char> {
    Parser::terminal(expected)
}

pub fn ascii_digit() -> Parser<char> {
    Parser::satisfy(|c| char::is_ascii_digit(&c))
}
pub fn digit(radix: u32) -> Parser<char> {
    Parser::satisfy(move |c| char::is_digit(c, radix))
}
pub fn numeric() -> Parser<char> {
    Parser::satisfy(char::is_numeric)
}
