mod stream;
pub use self::stream::Stream;

mod scanner;
pub use self::scanner::Scanner;

pub mod adapters;
pub use Either::{Left, Right};

pub(crate) use self::adapters::*;

#[macro_export]
macro_rules! or  {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($xs:tt)+) => {
        $x.or(or!($($xs)+))
    }
}

pub type Result<O, I> = std::result::Result<O, Error<I>>;

#[allow(type_alias_bounds)] // not yet
pub(crate) type Res<T: Scanner> = Result<T::Output, T::Input>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expected<E> {
    Any,
    Eof,
    Token(E),
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Error<E> {
    pos: usize,
    unexpected: Option<E>,
    expected: Expected<E>,
}

impl<E> Error<E> {
    pub(crate) fn new(pos: usize, unexpected: Option<E>, expected: Expected<E>) -> Self {
        Error {
            pos,
            unexpected,
            expected,
        }
    }
}

impl<E: std::fmt::Debug> std::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unexpected {:?}. expected {:?} at {}",
            self.unexpected, self.expected, self.pos
        )
    }
}

impl<E: std::fmt::Debug> std::error::Error for Error<E> {}

pub fn scan_with<F, A, B>(f: F) -> ScanWith<F, A, B>
where
    F: Fn(&mut Stream<A>) -> Result<B, A>,
{
    ScanWith::new(f)
}

pub fn any<T: Clone>() -> Any<T> {
    Any::new()
}

pub fn value<T: Clone, I>(x: T) -> Value<T, I> {
    Value::new(x)
}

pub fn eof<T: Clone>() -> Eof<T> {
    Eof::new()
}

pub fn expect<T: Clone, F: Fn(&T) -> bool>(f: F) -> Expect<T, F> {
    Expect::new(f)
}

pub fn fail<A: Clone, B>() -> Fail<A, B> {
    Fail::new()
}

pub fn token<A: Clone + PartialEq>(a: A) -> Token<A> {
    Token::new(a)
}

pub fn tokens<I, A>(iter: I) -> Tokens<A>
where
    I: IntoIterator<Item = A>,
    A: Clone + PartialEq,
{
    Tokens::new(iter.into_iter().collect())
}
