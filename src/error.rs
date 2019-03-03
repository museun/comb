use crate::*;

#[allow(type_alias_bounds)] // not yet
pub(crate) type Res<T: Scanner> = Result<T::Output, T::Input>;

/// Expected are invariants on the matching state of the combinator
///
/// [Any](crate::Expected::Any) is when *any* state was to be produced
///
/// [End](crate::Expected::End) is when the *end* was expected
///
/// [Token(E)](crate::Expected::Token) is when the Token, `e` was expected
///
/// [Unknown](crate::Expected::Unknown) is when an unknown state was entered (a catch-all)
#[derive(Debug, PartialEq, Clone)]
pub enum Expected<E> {
    Any,
    End,
    Token(E),
    Unknown,
}

/// Positional error produced from the [Expected](crate::Expected) states
#[derive(Debug, PartialEq, Clone)]
pub struct Error<E> {
    pub(crate) pos: usize,
    pub(crate) unexpected: Option<E>,
    pub(crate) expected: Expected<E>,
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
