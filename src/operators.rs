use crate::parser::Parser;
use std::ops::*;

// choiceと等価の演算子 |
impl<T: 'static> BitOr for Parser<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Parser::choice(self, rhs)
    }
}

// andと等価の演算子 &
impl<T: Clone + 'static> BitAnd for Parser<T> {
    // T & T -> Vec<T>
    type Output = Parser<Vec<T>>;
    fn bitand(self, rhs: Self) -> Parser<Vec<T>> {
        self.and(rhs)
    }
}
impl<T: Clone + 'static> BitAnd<Parser<T>> for Parser<Vec<T>> {
    // Vec<T> & T -> Vec<T>
    type Output = Parser<Vec<T>>;
    fn bitand(self, rhs: Parser<T>) -> Parser<Vec<T>> {
        self.and(rhs)
    }
}
impl<T: Clone + 'static> BitAnd<Parser<Vec<T>>> for Parser<T> {
    // T & Vec<T> -> Vec<T>
    type Output = Parser<Vec<T>>;
    fn bitand(self, rhs: Parser<Vec<T>>) -> Parser<Vec<T>> {
        self.and(rhs)
    }
}
