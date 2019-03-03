use crate::*;
use std::marker::PhantomData;

pub struct Then<A, F, B>(A, F, PhantomData<B>)
where
    A: Scanner,
    F: Fn(A::Output) -> B,
    B: Scanner<Input = A::Input>;

impl<A, F, B> Then<A, F, B>
where
    A: Scanner,
    F: Fn(A::Output) -> B,
    B: Scanner<Input = A::Input>,
{
    pub(crate) fn new(a: A, f: F) -> Self {
        Self(a, f, PhantomData)
    }
}

impl<A, F, B> Scanner for Then<A, F, B>
where
    A: Scanner,
    F: Fn(A::Output) -> B,
    B: Scanner<Input = A::Input>,
{
    type Input = A::Input;
    type Output = B::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        let ok = self.0.scan(stream)?;
        (self.1)(ok).scan(stream)
    }
}
