use crate::adapters::*;
use crate::stream::Stream;
use crate::{Expected, ScannerResult};

pub trait Scanner {
    type Input;
    type Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input>;

    fn map<T, F>(self, f: F) -> Map<T, Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> T,
    {
        Map::new(f, self)
    }

    fn or<T>(self, x: T) -> Or<Self, T>
    where
        Self: Sized,
        T: Scanner<Input = Self::Input, Output = Self::Output>,
    {
        Or::new(self, x)
    }

    fn and<T>(self, x: T) -> And<Self, T>
    where
        Self: Sized,
        T: Scanner<Input = Self::Input>,
    {
        And::new(self, x)
    }

    fn skip<T>(self, x: T) -> Skip<Self, T>
    where
        Self: Sized,
        T: Scanner<Input = Self::Input>,
    {
        Skip::new(self, x)
    }

    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
    {
        Optional::new(self)
    }

    fn then<B, F>(self, f: F) -> Then<Self, F, B>
    where
        Self: Sized,
        F: Fn(Self::Output) -> B,
        B: Scanner<Input = Self::Input>,
    {
        Then::new(self, f)
    }

    fn many(self) -> Many<Self>
    where
        Self: Sized,
    {
        Many::new(self, None, None)
    }

    fn many1(self) -> Many<Self>
    where
        Self: Sized,
    {
        Many::new(self, Some(1), None)
    }

    fn many_n(self, n: usize) -> Many<Self>
    where
        Self: Sized,
    {
        Many::new(self, Some(n), Some(n))
    }

    fn with<T>(self, x: T) -> With<Self, T>
    where
        Self: Sized,
        T: Scanner<Input = Self::Input>,
    {
        With::new(self, x)
    }

    fn value<T: Clone>(self, x: T) -> With<Self, Value<T, Self::Input>>
    where
        Self: Sized,
    {
        self.with(crate::value(x))
    }

    fn message(self, msg: Expected<Self::Input>) -> Message<Self>
    where
        Self: Sized,
        Self::Input: Clone,
    {
        Message::new(self, msg)
    }

    // need a:
    // message
    // error
}

impl<A: Scanner> Scanner for Box<A> {
    type Input = A::Input;
    type Output = A::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        (**self).scan(stream)
    }
}

impl<A: Scanner> Scanner for &A {
    type Input = A::Input;
    type Output = A::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        (**self).scan(stream)
    }
}

impl<A: Scanner> Scanner for &mut A {
    type Input = A::Input;
    type Output = A::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        (**self).scan(stream)
    }
}
