use crate::*;
use std::marker::PhantomData;

pub struct Map<O, T: Scanner, F: Fn(T::Output) -> O>(F, T, PhantomData<O>);

impl<O, T: Scanner, F: Fn(T::Output) -> O> Map<O, T, F> {
    pub(crate) fn new(f: F, x: T) -> Self {
        Self(f, x, PhantomData)
    }
}

impl<O, T: Scanner, F: Fn(T::Output) -> O> Scanner for Map<O, T, F> {
    type Input = T::Input;
    type Output = O;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        Ok((self.0)(self.1.scan(stream)?))
    }
}
