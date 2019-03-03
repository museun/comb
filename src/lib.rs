mod stream;
pub use self::stream::Stream;

mod scanner;
pub use self::scanner::Scanner;

pub mod adapters;
pub use Either::{Left, Right};

pub(crate) use self::adapters::*;

mod error;
pub(crate) use self::error::Res;
pub use self::error::{Error, Expected};

pub type Result<O, I> = std::result::Result<O, Error<I>>;

#[macro_export]
macro_rules! or  {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($xs:tt)+) => {
        $x.or(or!($($xs)+))
    }
}

pub fn scan_with<F, A, B>(f: F) -> ScanWith<F, A, B>
where
    F: Fn(&mut Stream<A>) -> Result<B, A>,
{
    ScanWith::new(f)
}

pub fn any<T>() -> Any<T>
where
    T: Clone,
{
    Any::new()
}

pub fn value<T, I>(x: T) -> Value<T, I>
where
    T: Clone,
{
    Value::new(x)
}

pub fn eof<T>() -> Eof<T>
where
    T: Clone,
{
    Eof::new()
}

pub fn expect<T, F>(f: F) -> Expect<T, F>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    Expect::new(f)
}

pub fn fail<A: Clone, B>() -> Fail<A, B> {
    Fail::new()
}

pub fn token<A>(a: A) -> Token<A>
where
    A: Clone + PartialEq,
{
    Token::new(a)
}

pub fn tokens<I, A>(iter: I) -> Tokens<A>
where
    I: IntoIterator<Item = A>,
    A: Clone + PartialEq,
{
    Tokens::new(iter.into_iter().collect())
}
