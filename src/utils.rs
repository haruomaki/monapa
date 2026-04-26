use crate::parser::Parser;

/// 特定の一文字を受理する
pub fn single(expected: char) -> Parser<char> {
    Parser::single(expected)
}
/// 特定の文字列を受理する
pub fn chunk(expected: impl AsRef<str> + 'static) -> Parser<String> {
    Parser::chunk(expected)
}
/// 残りの入力を全部Stringにして受理する
pub fn remnant() -> Parser<String> {
    Parser::remnant()
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
/// 空白を一文字だけ消費するパーサ
pub fn whitespace() -> Parser<char> {
    Parser::satisfy(char::is_whitespace)
}
/// 空白を消費するパーサ
pub fn ws() -> Parser<()> {
    (whitespace() * ..).void()
}

/// 指定の文字が出てくるまで読み込む
pub fn until(end: char) -> Parser<String> {
    (Parser::satisfy(move |c| c != end) * (..) << single(end)).map(|v| String::from_iter(v))
}

/// パーサのリストを受け取り、そのいずれかにマッチするかを調べるコンビネータ
pub fn choice<T, I>(parsers: I) -> Parser<T>
where
    T: Clone + 'static,
    I: IntoIterator,
    I::IntoIter: DoubleEndedIterator<Item = Parser<T>>,
{
    // 入力: [a, b, c]
    // 出力: ((empty.or(a)).or(b)).or(c)
    parsers
        .into_iter()
        .fold(Parser::<T>::empty(), |accum, p| accum.or(p))
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

/// 遅延評価を書くマクロ
#[macro_export]
macro_rules! lazy {
    ($name:ident) => {
        Parser::lazy(|| $name())
    };
}
