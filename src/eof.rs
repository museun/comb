use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Eof<T: Clone>(PhantomData<T>);

impl<T: Clone> Eof<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Clone> Scanner for Eof<T> {
    type Input = T;
    type Output = ();

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        if let Some(tok) = stream.peek() {
            Err(Error::new(stream.pos(), Some(tok), Expected::Eof))
        } else {
            Ok(())
        }
    }
}
