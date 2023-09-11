use crate::parser::Parser;

// choiceと等価の演算子 <|>
impl<T: 'static> std::ops::BitOr for Parser<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Parser::choice(self, rhs)
    }
}
