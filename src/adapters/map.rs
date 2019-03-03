use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Map<O, T, F>(F, T, PhantomData<O>)
where
    T: Scanner,
    F: Fn(T::Output) -> O;

impl<O, T, F> Map<O, T, F>
where
    T: Scanner,
    F: Fn(T::Output) -> O,
{
    pub(crate) fn new(f: F, x: T) -> Self {
        Self(f, x, PhantomData)
    }
}

impl<O, T, F> Scanner for Map<O, T, F>
where
    T: Scanner,
    F: Fn(T::Output) -> O,
{
    type Input = T::Input;
    type Output = O;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        Ok((self.0)(self.1.scan(stream)?))
    }
}
