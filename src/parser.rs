use std::rc::Rc;

#[derive(Debug)]
pub enum ParseError {
    DeliberateFailure,
    WrongSingle(char, char),
    WrongChunk(String, String),
    ChoiceMismatch(Box<ParseError>, Box<ParseError>),
    SatisfyError,
    RepeatError,
    IncompleteParse(Box<dyn std::any::Any>),
    IterationError,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for ParseError {}

pub type ParseResult<T> = Result<T, ParseError>;

/// Structure representing a parser definition. Call the member function `parse`.
#[derive(Clone)]
pub struct Parser<T: Clone + 'static> {
    _parse: Rc<dyn Fn(&mut std::str::Chars) -> ParseResult<T>>,
}

// For internal use only
fn new<T: Clone + 'static>(
    _parse: impl Fn(&mut std::str::Chars) -> ParseResult<T> + 'static,
) -> Parser<T> {
    Parser {
        _parse: Rc::new(_parse),
    }
}

impl<T: Clone + 'static> Parser<T> {
    /// Parse a string with a current parser.
    pub fn parse(&self, input: impl AsRef<str>) -> ParseResult<T> {
        let mut iter = input.as_ref().chars();
        let ast = (self._parse)(&mut iter)?;
        if iter.next() == None {
            Ok(ast)
        } else {
            Err(ParseError::IncompleteParse(Box::new(ast)))
        }
    }

    // ------------------------------
    // Monad functionality
    // ------------------------------

    // モナドのbind関数。連接を表す
    pub fn bind<S: Clone + 'static, F: Fn(T) -> Parser<S> + 'static>(self, f: F) -> Parser<S> {
        new(move |iter| {
            let ast = (self._parse)(iter)?;
            let par = f(ast);
            (par._parse)(iter)
        })
    }

    // map
    pub fn map<S: Clone + 'static, F: Fn(T) -> S + 'static>(self, f: F) -> Parser<S> {
        new(move |iter| {
            let ast = (self._parse)(iter)?;
            Ok(f(ast))
        })
    }

    // おまけでAlternativeとしての要件。必ず失敗するパーサ。
    pub fn empty() -> Self {
        new(|_| Err(ParseError::DeliberateFailure))
    }

    // `return` function
    pub fn ret(value: T) -> Self {
        new(move |_| Ok(value.clone()))
    }

    // 選択を表すコンビネータ
    pub fn choice(self, other: Self) -> Self {
        // INFO: Errのときだけ処理を続行する「?」演算子があればもっと簡潔に書ける？（でもiter_backupは無理かも）
        new(move |iter| {
            let iter_backup = iter.clone();
            match (self._parse)(iter) {
                Ok(res) => Ok(res),
                Err(e1) => {
                    *iter = iter_backup;
                    match (other._parse)(iter) {
                        Ok(res) => Ok(res),
                        Err(e2) => Err(ParseError::ChoiceMismatch(Box::new(e1), Box::new(e2))),
                    }
                }
            }
        })
    }

    // 繰り返しを表すコンビネータ
    pub fn repeat(self, min: Option<usize>, max: Option<usize>) -> Parser<Vec<T>> {
        new(move |iter| {
            let mut count = 1;
            let mut asts = vec![];
            while match max {
                Some(v) => count <= v,
                None => true,
            } {
                let iter_backup = iter.clone();
                let res = (self._parse)(iter);
                if let Ok(ast) = res {
                    asts.push(ast);
                    count += 1;
                } else {
                    *iter = iter_backup;
                    break;
                }
            }

            if min.is_some() && asts.len() < min.unwrap() {
                Err(ParseError::RepeatError)
            } else {
                Ok(asts)
            }
        })
    }

    pub fn and<U: Clone + 'static, S: Clone + 'static>(self, rhs: Parser<S>) -> Parser<Vec<U>>
    where
        Parser<T>: Into<Parser<Vec<U>>>,
        Parser<S>: Into<Parser<Vec<U>>>,
    {
        let lhs: Parser<Vec<U>> = self.into();
        let rhs: Parser<Vec<U>> = rhs.into();
        lhs.concat(rhs)
    }
}

impl<T: Clone + 'static> Parser<Vec<T>> {
    pub fn concat(self, rhs: Self) -> Parser<Vec<T>> {
        new(move |iter| {
            let ast_left = (self._parse)(iter)?;
            let ast_right = (rhs._parse)(iter)?;
            Ok(vec![ast_left, ast_right].concat())
        })
    }
}

impl<T: Clone + 'static> Into<Parser<Vec<T>>> for Parser<T> {
    fn into(self) -> Parser<Vec<T>> {
        new(move |iter| {
            let ast = (self._parse)(iter)?;
            Ok(vec![ast])
        })
    }
}

impl Parser<()> {
    // ------------------------------
    // Character parsing operations
    // ------------------------------

    // 特定の一文字をパースしてその文字を返すパーサ
    pub fn single(expected: char) -> Parser<char> {
        new(move |iter| match iter.next() {
            Some(c) => match c == expected {
                true => return Ok(c),
                false => Err(ParseError::WrongSingle(c, expected)),
            },
            None => Err(ParseError::IterationError),
        })
    }

    pub fn chunk(expected: impl AsRef<str> + 'static) -> Parser<String> {
        new(move |iter| {
            let mut found = vec![];
            for ex in expected.as_ref().chars() {
                if let Some(c) = iter.next() {
                    found.push(c);
                    if c != ex {
                        return Err(ParseError::WrongChunk(
                            found.iter().collect(),
                            expected.as_ref().to_string(),
                        ));
                    }
                } else {
                    return Err(ParseError::IterationError);
                }
            }
            Ok(expected.as_ref().to_string())
        })
    }

    // TODO: boodでなくResultにして、エラーを詳細化
    pub fn satisfy(f: impl Fn(char) -> bool + 'static) -> Parser<char> {
        new(move |iter| match iter.next() {
            Some(c) => match f(c) {
                true => return Ok(c),
                false => Err(ParseError::SatisfyError),
            },
            None => Err(ParseError::IterationError),
        })
    }
}
