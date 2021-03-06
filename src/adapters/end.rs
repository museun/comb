use crate::*;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct End<T: Clone>(PhantomData<T>);

impl<T: Clone> End<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Clone> Scanner for End<T> {
    type Input = T;
    type Output = ();

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        if let Some(tok) = stream.peek() {
            let err = ErrorBuilder::new(stream.pos())
                .expected(ExpectedKind::End)
                .unexpected(tok)
                .build();
            return Err(err);
        }

        Ok(())
    }
}
