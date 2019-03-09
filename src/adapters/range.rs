use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Range<T>(std::ops::Range<T>, PhantomData<T>)
where
    T: Clone + PartialOrd + std::fmt::Debug;

impl<T> Range<T>
where
    T: Clone + PartialOrd + std::fmt::Debug,
{
    pub fn new(range: std::ops::Range<T>) -> Self {
        Self(range, PhantomData)
    }
}

impl<T> Scanner for Range<T>
where
    T: Clone + PartialOrd + std::fmt::Debug,
{
    type Input = T;
    type Output = T;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        let val = crate::must_peek(stream)?;
        if val < self.0.start || val > self.0.end {
            // TODO say which direction
            let err = ErrorBuilder::new(stream.pos())
                .unexpected(val)
                .message(format!("outside of range: {:?}", self.0))
                .build();
            return Err(err);
        }

        stream.next();
        Ok(val)
    }
}
