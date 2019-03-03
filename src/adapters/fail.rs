use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Fail<A: Clone, B>(PhantomData<(A, B)>);

impl<A: Clone, B> Fail<A, B> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl<A: Clone, B> Scanner for Fail<A, B> {
    type Input = A;
    type Output = B;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        Err(Error::new(
            stream.pos(),
            stream.peek().map(Some).unwrap_or_else(|| None),
            Expected::Unknown,
        ))
    }
}
