use crate::*;
use std::marker::PhantomData;

pub struct Expect<T: Clone, F: Fn(&T) -> bool>(F, PhantomData<T>);

impl<T: Clone, F: Fn(&T) -> bool> Expect<T, F> {
    pub(crate) fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<T: Clone, F: Fn(&T) -> bool> Scanner for Expect<T, F> {
    type Input = T;
    type Output = T;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        let val = stream
            .peek()
            .ok_or_else(|| ScannerError::new(stream.pos(), None, Expected::Unknown))?;

        if self.0(&val) {
            stream.next();
            Ok(val)
        } else {
            Err(ScannerError::new(
                stream.pos(),
                Some(val),
                Expected::Unknown,
            ))
        }
    }
}
