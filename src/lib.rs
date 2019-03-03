mod stream;
pub use self::stream::Stream;

mod scanner;
pub use self::scanner::Scanner;

/// Collection of all of the available combinators
pub mod adapters;

pub use Either::{Left, Right};

pub(crate) use self::adapters::*;

mod error;
pub(crate) use self::error::Res;
pub use self::error::{Error, Expected};

/// Result is a wrapped result type. It wraps an error with the internal [Error](crate::Error) tracking
pub type Result<O, I> = std::result::Result<O, Error<I>>;

/// Convience macro for [or](crate::Scanner::or)ing together many combinators
#[macro_export]
macro_rules! or  {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($xs:tt)+) => {
        $x.or(or!($($xs)+))
    }
}

/// Scans the stream with a production function
pub fn scan_with<F, A, B>(f: F) -> ScanWith<F, A, B>
where
    F: Fn(&mut Stream<A>) -> Result<B, A>,
{
    ScanWith::new(f)
}

/// Produce any token
pub fn any<T>() -> Any<T>
where
    T: Clone,
{
    Any::new()
}

/// Produce a specific value
pub fn value<T, I>(x: T) -> Value<T, I>
where
    T: Clone,
{
    Value::new(x)
}

/// Produce the end of input
pub fn end<T>() -> End<T>
where
    T: Clone,
{
    End::new()
}

/// Expect the clsoure, producing the value or an error
pub fn expect<T, F>(f: F) -> Expect<T, F>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    Expect::new(f)
}

/// Produce a failure
pub fn fail<A: Clone, B>() -> Fail<A, B> {
    Fail::new()
}

/// Produce a single token
pub fn token<A>(a: A) -> Token<A>
where
    A: Clone + PartialEq,
{
    Token::new(a)
}

/// Produce an collection of tokens
pub fn tokens<I, A>(iter: I) -> Tokens<A>
where
    I: IntoIterator<Item = A>,
    A: Clone + PartialEq,
{
    Tokens::new(iter.into_iter().collect())
}
