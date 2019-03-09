use crate::*;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[allow(type_alias_bounds)] // not yet
pub(crate) type Res<T: Scanner> = Result<T::Output, T::Input>;

#[derive(Debug, Clone)]
pub enum ExpectedKind<E> {
    Any,
    End,
    Token(E),
    Message(String),
    Unknown,
}

impl<E> From<E> for ExpectedKind<E> {
    fn from(d: E) -> Self {
        ExpectedKind::Token(d)
    }
}

impl<E> Display for ExpectedKind<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ExpectedKind::Any => write!(f, "any token"),
            ExpectedKind::End => write!(f, "end of input"),
            ExpectedKind::Token(tok) => write!(f, "'{}'", tok),
            ExpectedKind::Message(tok) => write!(f, "'{}'", tok),
            ExpectedKind::Unknown => write!(f, "unknown token"),
        }
    }
}

#[derive(Debug)]
pub struct Error<L, R> {
    pub pos: usize,
    pub expected: Option<ExpectedKind<L>>,
    pub unexpected: Option<ExpectedKind<R>>,
    pub msg: Option<String>,
}

pub(crate) struct ErrorBuilder<L, R> {
    pos: usize,
    expected: Option<ExpectedKind<L>>,
    unexpected: Option<ExpectedKind<R>>,
    msg: Option<String>,
}

impl<L, R> Default for ErrorBuilder<L, R> {
    fn default() -> Self {
        Self {
            pos: 0,
            expected: None,
            unexpected: None,
            msg: None,
        }
    }
}

impl<L, R> ErrorBuilder<L, R> {
    pub fn new(pos: usize) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }

    pub fn expected<E>(mut self, expected: E) -> Self
    where
        E: Into<ExpectedKind<L>>,
    {
        self.expected = Some(expected.into());
        self
    }

    pub fn unexpected<E>(mut self, unexpected: E) -> Self
    where
        E: Into<ExpectedKind<R>>,
    {
        self.unexpected = Some(unexpected.into());
        self
    }

    pub fn message<S>(mut self, reason: S) -> Self
    where
        S: ToString,
    {
        self.msg = Some(reason.to_string());
        self
    }

    pub fn build(self) -> Error<L, R> {
        Error {
            pos: self.pos,
            expected: self.expected,
            unexpected: self.unexpected,
            msg: self.msg,
        }
    }
}

impl<L, R> Display for Error<L, R>
where
    L: Display,
    R: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(e) = &self.expected {
            write!(f, "expected {}.", e)?;
        }

        if let Some(e) = &self.unexpected {
            write!(f, "unexpected {}.", e)?
        }

        write!(f, " at {}.", self.pos)?;

        if let Some(m) = &self.msg {
            write!(f, " ({})", m)?;
        }

        Ok(())
    }
}

impl<L, R> std::error::Error for Error<L, R>
where
    L: Display + Debug,
    R: Display + Debug,
{
}
