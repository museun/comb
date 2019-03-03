mod stream;
pub use self::stream::Stream;

mod scanner;
pub use self::scanner::Scanner;

pub mod adapters;

#[macro_export]
macro_rules! or  {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($xs:tt)+) => {
        $x.or(or!($($xs)+))
    }
}

pub type ScannerResult<O, I> = std::result::Result<O, Error<I>>;

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

pub fn scan_with<F, A, B>(f: F) -> self::adapters::ScanWith<F, A, B>
where
    F: Fn(&mut Stream<A>) -> ScannerResult<B, A>,
{
    self::adapters::ScanWith::new(f)
}

pub fn any<T: Clone>() -> self::adapters::Any<T> {
    self::adapters::Any::new()
}

pub fn value<T: Clone, I>(x: T) -> self::adapters::Value<T, I> {
    self::adapters::Value::new(x)
}

pub fn eof<T: Clone>() -> self::adapters::Eof<T> {
    self::adapters::Eof::new()
}

pub fn expect<T: Clone, F: Fn(&T) -> bool>(f: F) -> self::adapters::Expect<T, F> {
    self::adapters::Expect::new(f)
}

pub fn fail<A: Clone, B>() -> self::adapters::Fail<A, B> {
    self::adapters::Fail::new()
}

pub fn token<A: Clone + PartialEq>(a: A) -> self::adapters::Token<A> {
    self::adapters::Token::new(a)
}

pub fn tokens<I, A>(iter: I) -> self::adapters::Tokens<A>
where
    I: IntoIterator<Item = A>,
    A: Clone + PartialEq,
{
    self::adapters::Tokens::new(iter.into_iter().collect())
}

pub use Either::*;
pub enum Either<L, R>
where
    L: Scanner,
    R: Scanner<Input = L::Input, Output = L::Output>,
{
    Left(L),
    Right(R),
}

impl<L, R> Scanner for Either<L, R>
where
    L: Scanner,
    R: Scanner<Input = L::Input, Output = L::Output>,
{
    type Input = L::Input;
    type Output = L::Output;
    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        match self {
            Either::Left(left) => left.scan(stream),
            Either::Right(right) => right.scan(stream),
        }
    }
}
