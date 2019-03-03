use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Value<T: Clone, I>(T, PhantomData<I>);

impl<T: Clone, I> Value<T, I> {
    pub(crate) fn new(x: T) -> Self {
        Self(x, PhantomData)
    }
}

impl<T: Clone, I> Scanner for Value<T, I> {
    type Input = I;
    type Output = T;

    fn scan(&self, _: &mut Stream<Self::Input>) -> Res<Self> {
        Ok(self.0.clone())
    }
}
