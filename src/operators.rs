use crate::parser::Parser;
use std::ops::*;

// choiceと等価の演算子 |
impl<T: Clone + 'static> BitOr for Parser<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Parser::choice(self, rhs)
    }
}

// thenと等価な演算子 >>
impl<T: Clone + 'static, S: Clone + 'static> Shr<Parser<S>> for Parser<T> {
    type Output = Parser<S>;
    fn shr(self, rhs: Parser<S>) -> Parser<S> {
        self.then(rhs)
    }
}

// skipと等価な演算子 <<
impl<T: Clone + 'static, S: Clone + 'static> Shl<Parser<S>> for Parser<T> {
    type Output = Parser<T>;
    fn shl(self, rhs: Parser<S>) -> Parser<T> {
        self.skip(rhs)
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
impl<T: Clone + 'static> Mul<RangeFull> for Parser<T> {
    // Parser<T> * (..)
    type Output = Parser<Vec<T>>;
    fn mul(self, _: RangeFull) -> Self::Output {
        self.repeat(None, None)
    }
}
impl<T: Clone + 'static> Mul<RangeFrom<usize>> for Parser<T> {
    // Parser<T> * (n..)
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: RangeFrom<usize>) -> Self::Output {
        self.repeat(Some(rhs.start), None)
    }
}
impl<T: Clone + 'static> Mul<RangeTo<usize>> for Parser<T> {
    // Parser<T> * (..m)
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: RangeTo<usize>) -> Self::Output {
        self.repeat(None, Some(rhs.end))
    }
}
impl<T: Clone + 'static> Mul<Range<usize>> for Parser<T> {
    // Parser<T> * (n..m)
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: Range<usize>) -> Self::Output {
        self.repeat(Some(rhs.start), Some(rhs.end))
    }
}
impl<T: Clone + 'static> Mul<usize> for Parser<T> {
    // Parser<T> * n
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: usize) -> Self::Output {
        self.repeat(Some(rhs), Some(rhs))
    }
}
