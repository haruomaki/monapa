use crate::parser::Parser;
use std::ops::*;

// orと等価な演算子 |
impl<T: Clone> BitOr for Parser<T> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        self.or(rhs)
    }
}

// andと等価な演算子 &
impl<T: Clone, S: Clone> BitAnd<Parser<S>> for Parser<T> {
    type Output = Parser<(T, S)>;
    fn bitand(self, rhs: Parser<S>) -> Parser<(T, S)> {
        self.and(rhs)
    }
}

// thenと等価な演算子 >>
impl<T: Clone, S: Clone> Shr<Parser<S>> for Parser<T> {
    type Output = Parser<S>;
    fn shr(self, rhs: Parser<S>) -> Parser<S> {
        self.then(rhs)
    }
}

// skipと等価な演算子 <<
impl<T: Clone, S: Clone> Shl<Parser<S>> for Parser<T> {
    type Output = Parser<T>;
    fn shl(self, rhs: Parser<S>) -> Parser<T> {
        self.skip(rhs)
    }
}

// concatと等価な演算子 +
impl<T: Clone> Add for Parser<Vec<T>> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.concat(rhs)
    }
}

// 繰り返しの演算
impl<T: Clone> Mul<RangeFull> for Parser<T> {
    // Parser<T> * (..)
    type Output = Parser<Vec<T>>;
    fn mul(self, _: RangeFull) -> Self::Output {
        self.repeat(None, None)
    }
}
impl<T: Clone> Mul<RangeFrom<usize>> for Parser<T> {
    // Parser<T> * (n..)
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: RangeFrom<usize>) -> Self::Output {
        self.repeat(Some(rhs.start), None)
    }
}
impl<T: Clone> Mul<RangeTo<usize>> for Parser<T> {
    // Parser<T> * (..m)
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: RangeTo<usize>) -> Self::Output {
        self.repeat(None, Some(rhs.end))
    }
}
impl<T: Clone> Mul<Range<usize>> for Parser<T> {
    // Parser<T> * (n..m)
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: Range<usize>) -> Self::Output {
        self.repeat(Some(rhs.start), Some(rhs.end))
    }
}
impl<T: Clone> Mul<usize> for Parser<T> {
    // Parser<T> * n
    type Output = Parser<Vec<T>>;
    fn mul(self, rhs: usize) -> Self::Output {
        self.repeat(Some(rhs), Some(rhs))
    }
}
