use crate::*;
use std::marker::PhantomData;

pub struct Fail<A: Clone, B>(PhantomData<(A, B)>);

impl<A: Clone, B> Fail<A, B> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl<A: Clone, B> Scanner for Fail<A, B> {
    type Input = A;
    type Output = B;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        Err(ScannerError::new(
            stream.pos(),
            stream.peek().map(Some).unwrap_or_else(|| None),
            Expected::Unknown,
        ))
    }
}
