use crate::*;
use std::marker::PhantomData;

// TODO provide a ToExpected trait so this can describe what 'f' does
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
        let val = crate::must_peek(stream)?;
        if self.0(&val) {
            stream.next();
            return Ok(val);
        }

        let err = ErrorBuilder::new(stream.pos()).unexpected(val).build();
        Err(err)
    }
}
