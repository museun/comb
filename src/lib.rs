mod stream;
pub use self::stream::Stream;

mod scanner;
pub use self::scanner::Scanner;

/// Collection of all of the available combinators
pub mod adapters;

pub use Either::{Left, Right};

pub(crate) use self::adapters::*;

mod error;
pub use self::error::{Error, ExpectedKind};
pub(crate) use self::error::{ErrorBuilder, Res};

/// Result is a wrapped result type. It wraps an error with the internal [Error](crate::Error) tracking
pub type Result<O, I> = std::result::Result<O, Error<I, I>>;

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

pub fn ignore<T>(x: T) -> Ignore<T>
where
    T: Scanner,
{
    Ignore::new(x)
}

/// Expect the closure, producing the value or an error
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

pub fn string<S>(input: S) -> StringToken
where
    S: ToString,
{
    StringToken::new(input.to_string(), true)
}

pub fn string_no_case<S>(input: S) -> StringToken
where
    S: ToString,
{
    StringToken::new(input.to_string(), false)
}

pub fn whitespace() -> impl Scanner<Input = char, Output = char> {
    expect(char::is_ascii_whitespace)
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

// TODO make this use RangeBounds once `contains` lands in stable
/// Produce a range of tokens
pub fn range<T>(range: std::ops::Range<T>) -> Range<T>
where
    T: Clone + PartialOrd + std::fmt::Debug,
{
    Range::new(range)
}

/// Concats `x` with `xs`
pub trait Concat<T>
where
    T: std::iter::FromIterator<Self>,
    Self: Sized,
{
    fn concat<I>(&self, tail: I) -> T
    where
        Self: Sized + Clone,
        I: IntoIterator<Item = Self>,
    {
        std::iter::once(self.clone())
            .chain(tail.into_iter())
            .collect()
    }
}

impl<T, U> Concat<U> for T where U: std::iter::FromIterator<T> {}
// TODO concat tuples (figure out how to express (a,b,) (trailing comma is important))

pub fn digit() -> impl Scanner<Input = char, Output = char> {
    range('0'..'9') // probably works
}

pub fn letter() -> impl Scanner<Input = char, Output = char> {
    range('a'..'z').or(range('A'..'Z'))
}

#[derive(Debug, Clone, Copy)]
pub enum NumberWidth {
    W8,
    W16,
    W32,
    W64,
    W128,
    WSize,
}

pub fn number<T>() -> impl Scanner<Input = char, Output = T>
where
    T: std::str::FromStr + Clone,
    T::Err: std::fmt::Display,
{
    digit()
        .many1()
        .map(|d| d.into_iter().collect::<String>().parse::<T>())
        .then(|x| match x {
            Ok(d) => Either::Right(value(d)),
            Err(err) => Either::Left(fail().message(format!("invalid number: {}", err))),
        })
}
