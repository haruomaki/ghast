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

// 繰り返しの演算
impl<T: 'static, R: RangeBounds<usize>> Mul<R> for Parser<T> {
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: R) -> Self::Output {
        let min = match rhs.start_bound() {
            Bound::Included(u) => Some(*u),
            Bound::Excluded(u) => Some(*u + 1),
            Bound::Unbounded => None,
        };
        let max = match rhs.end_bound() {
            Bound::Included(u) => Some(*u),
            Bound::Excluded(u) => Some(*u - 1),
            Bound::Unbounded => None,
        };
        self.many(min, max)
    }
}
// impl<T: 'static> Mul<usize> for Parser<T> {
//     type Output = Parser<Vec<T>>;
//     fn mul(self, rhs: usize) -> Self::Output {
//         self.many(Some(rhs), Some(rhs))
//     }
// }
