use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Any<T: Clone>(PhantomData<T>);

impl<T: Clone> Any<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Clone> Scanner for Any<T> {
    type Input = T;
    type Output = T;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        let val = crate::must_peek(stream)?;
        stream.next();
        Ok(val)
    }
}
