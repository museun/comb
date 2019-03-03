use crate::*;
use std::marker::PhantomData;

pub struct Expect<T, F>(F, PhantomData<T>)
where
    T: Clone,
    F: Fn(&T) -> bool;

impl<T, F> Expect<T, F>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    pub(crate) fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<T, F> Scanner for Expect<T, F>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    type Input = T;
    type Output = T;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        let val = stream
            .peek()
            .ok_or_else(|| Error::new(stream.pos(), None, Expected::Unknown))?;

        if self.0(&val) {
            stream.next();
            Ok(val)
        } else {
            Err(Error::new(stream.pos(), Some(val), Expected::Unknown))
        }
    }
}
