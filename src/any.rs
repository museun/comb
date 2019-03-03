use crate::*;
use std::marker::PhantomData;

pub struct Any<T: Clone>(PhantomData<T>);

impl<T: Clone> Any<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Clone> Scanner for Any<T> {
    type Input = T;
    type Output = T;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        let val = stream
            .peek()
            .ok_or_else(|| ScannerError::new(stream.pos(), None, Expected::Any))?;
        stream.next();
        Ok(val)
    }
}
