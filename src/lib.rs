mod stream;
pub use self::stream::Stream;

mod scanner;
pub use self::scanner::Scanner;

mod and;
mod many;
mod map;
mod optional;
mod or;
mod skip;
mod then;

mod any;
mod eof;
mod expect;
mod fail;
mod scanwith;
mod token;
mod tokens;

pub mod adapters {
    pub use super::and::*;
    pub use super::many::*;
    pub use super::map::*;
    pub use super::optional::*;
    pub use super::or::*;
    pub use super::skip::*;
    pub use super::then::*;

    pub use super::any::*;
    pub use super::eof::*;
    pub use super::expect::*;
    pub use super::fail::*;
    pub use super::scanwith::*;
    pub use super::token::*;
    pub use super::tokens::*;
}

#[macro_export]
macro_rules! or  {
    ($e:expr) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn tester<A: Scanner, I>(scanner: A, table: I)
    where
        I: IntoIterator<Item = (Vec<A::Input>, ScannerResult<A::Output, A::Input>, usize)>,
        A::Input: PartialEq + std::fmt::Debug,
        A::Output: PartialEq + std::fmt::Debug,
    {
        for (input, result, pos) in table {
            let mut stream = Stream::new(input);
            assert_eq!(result, scanner.scan(&mut stream));
            assert_eq!(pos, stream.pos())
        }
    }

    #[test]
    fn map_test() {
        tester(
            token(1).map(|x| x + 1),
            vec![
                (vec![1], Ok(2), 1),
                (vec![2], Err(Error::new(0, Some(2), Expected::Token(1))), 0),
            ],
        )
    }

    #[test]
    fn or_test() {
        tester(
            token(1).or(token(2)),
            vec![
                (vec![1], Ok(1), 1),
                (vec![2], Ok(2), 1),
                (vec![3], Err(Error::new(0, Some(3), Expected::Token(2))), 0),
            ],
        );

        tester(
            tokens(vec![1, 2]).or(tokens(vec![1, 3])),
            vec![
                (
                    vec![1, 3],
                    Err(Error::new(1, Some(3), Expected::Token(2))),
                    1,
                ),
                (
                    vec![1, 1, 3],
                    Err(Error::new(1, Some(1), Expected::Token(2))),
                    1,
                ),
            ],
        );
    }

    #[test]
    #[ignore]
    fn and_test() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn skip_test() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn optional_test() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn then_test() {
        unimplemented!()
    }

    #[test]
    #[ignore]
    fn many_test() {
        unimplemented!()
    }
}
