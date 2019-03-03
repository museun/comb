use crate::*;

/// This trait is the core of the combinatorial sequencing
///
/// The [scan](Scanner::scan) method must be implemented, it takes a
/// [Stream](crate::stream::Stream) and produces a
/// [Result](crate::error::Result) of [Self::Output](Scanner::Output) from the
/// [Self::Input](Scanner::Input)
///
/// [Input](Scanner::Input) is the input type
///
/// [Output](Scanner::Output) is the production type
pub trait Scanner {
    type Input;
    type Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Result<Self::Output, Self::Input>;

    /// Maps a function over the sequence
    fn map<T, F>(self, f: F) -> Map<T, Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> T,
    {
        Map::new(f, self)
    }

    /// Produces an `or` squence with the current and next combinator
    fn or<T>(self, x: T) -> Or<Self, T>
    where
        Self: Sized,
        T: Scanner<Input = Self::Input, Output = Self::Output>,
    {
        Or::new(self, x)
    }

    /// Produces an `and` sequence with the current and next combinator
    fn and<T>(self, x: T) -> And<Self, T>
    where
        Self: Sized,
        T: Scanner<Input = Self::Input>,
    {
        And::new(self, x)
    }

    /// Skips the next element if it matches the type
    fn skip<T>(self, x: T) -> Skip<Self, T>
    where
        Self: Sized,
        T: Scanner<Input = Self::Input>,
    {
        Skip::new(self, x)
    }

    /// Produces a sequence where this element is optional
    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
    {
        Optional::new(self)
    }

    /// When a successful production has happened, run a function producing a new sequence
    fn then<B, F>(self, f: F) -> Then<Self, F, B>
    where
        Self: Sized,
        F: Fn(Self::Output) -> B,
        B: Scanner<Input = Self::Input>,
    {
        Then::new(self, f)
    }

    /// Repeat the previous combinator until an error is produced
    fn many(self) -> Many<Self>
    where
        Self: Sized,
    {
        Many::new(self, None, None)
    }

    /// Repeat the previous combinator *at least once* until an error is produced
    fn many1(self) -> Many<Self>
    where
        Self: Sized,
    {
        Many::new(self, Some(1), None)
    }

    /// Repeat the previous combinator *at least `n`* times until an error is produced
    fn many_n(self, n: usize) -> Many<Self>
    where
        Self: Sized,
    {
        Many::new(self, Some(n), Some(n))
    }

    /// Chains two combinators together
    fn with<T>(self, x: T) -> With<Self, T>
    where
        Self: Sized,
        T: Scanner<Input = Self::Input>,
    {
        With::new(self, x)
    }

    /// Produces a value from this sequence
    fn value<T: Clone>(self, x: T) -> With<Self, Value<T, Self::Input>>
    where
        Self: Sized,
    {
        self.with(crate::value(x))
    }

    /// Produces an error message at this point in the sequence
    fn message(self, msg: Expected<Self::Input>) -> Message<Self>
    where
        Self: Sized,
        Self::Input: Clone,
    {
        Message::new(self, msg)
    }

    /// Try to do the combinator, otherwise rollback to the last successful one
    fn attempt(self) -> Attempt<Self>
    where
        Self: Sized,
    {
        Attempt::new(self)
    }
}

impl<A: Scanner> Scanner for Box<A> {
    type Input = A::Input;
    type Output = A::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        (**self).scan(stream)
    }
}

impl<A: Scanner> Scanner for &A {
    type Input = A::Input;
    type Output = A::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        (**self).scan(stream)
    }
}

impl<A: Scanner> Scanner for &mut A {
    type Input = A::Input;
    type Output = A::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        (**self).scan(stream)
    }
}
