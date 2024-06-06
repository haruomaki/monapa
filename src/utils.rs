use crate::parser::Parser;

pub fn single(expected: char) -> Parser<char> {
    Parser::single(expected)
}
pub fn chunk(expected: impl AsRef<str> + 'static) -> Parser<()> {
    Parser::chunk(expected)
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
pub fn alphabetic() -> Parser<char> {
    Parser::satisfy(char::is_alphabetic)
}
pub fn alphanumeric() -> Parser<char> {
    Parser::satisfy(char::is_alphanumeric)
}
pub fn whitespace() -> Parser<char> {
    Parser::satisfy(char::is_whitespace)
}

pub fn option<T: Clone + 'static>(p: Parser<T>) -> Parser<Option<T>> {
    p.map(|ast| Some(ast)) | Parser::ret(None)
}

impl<T: Clone + 'static> Parser<T> {
    pub fn separated_by<S: Clone + 'static>(self, p: Parser<S>) -> Parser<Vec<T>> {
        self.clone().bind(move |head| {
            let slf = self.clone();
            let tail_parser = p.clone().bind(move |_| slf.clone()) * (..);
            tail_parser.bind(move |tail| Parser::ret(vec![vec![head.clone()], tail].concat()))
        })
    }
}

// https://blog-dry.com/entry/2020/12/25/130250#do-記法
// https://zenn.dev/heppoko_quasar/articles/df8e0aed2c088e
#[macro_export]
macro_rules! pdo {
    ($($t:tt)*) => {
        $crate::pdo_with_env!{~~ $($t)*}
    };
}

#[macro_export]
macro_rules! pdo_with_env {
    // 値を取り出してbindする（>>=）
    (~$($env:ident)*~ $i:ident <- $e:expr; $($t:tt)*) => {
        $crate::Parser::bind($e, move |$i| {
            $(let $env = $env.clone();)*
            $crate::pdo_with_env!{~$($env)* $i~ $($t)*}
        })
    };

    // モナドから取り出した値を使わない場合（>>）
    (~$($env:ident)*~ $e:expr; $($t:tt)*) => {
        $crate::Parser::bind($e, move |_| {
            $(let $env = $env.clone();)*
            $crate::pdo_with_env!{~$($env)*~ $($t)*}
        })
    };

    // let文
    (~$($env:ident)*~ let $i:ident = $e:expr; $($t:tt)*) => {
        let $i = $e;
        $crate::pdo_with_env!{~$($env)* $i~ $($t)*}
    };

    // return関数
    (~$($env:ident)*~ return $e:expr) => {
        $crate::Parser::ret($e)
    };

    // returnでなくモナドを直接指定して返す
    (~$($env:ident)*~ $e:expr) => {
        $e
    };
}
