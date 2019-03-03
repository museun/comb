use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ScanWith<F, A, B>(F, PhantomData<(A, B)>)
where
    F: Fn(&mut Stream<A>) -> ScannerResult<B, A>;

impl<F, A, B> ScanWith<F, A, B>
where
    F: Fn(&mut Stream<A>) -> ScannerResult<B, A>,
{
    pub(crate) fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F, A, B> Scanner for ScanWith<F, A, B>
where
    F: Fn(&mut Stream<A>) -> ScannerResult<B, A>,
{
    type Input = A;
    type Output = B;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        (self.0)(stream)
    }
}
