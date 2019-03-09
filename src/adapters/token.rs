use crate::*;

#[derive(Debug, Clone)]
pub struct Token<A: Clone + PartialEq>(A);

impl<A: Clone + PartialEq> Token<A> {
    pub(crate) fn new(a: A) -> Self {
        Self(a)
    }
}

impl<A: Clone + PartialEq> Scanner for Token<A> {
    type Input = A;
    type Output = A;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        let val = crate::must_peek(stream)?;
        if val == self.0 {
            stream.next();
            return Ok(val);
        }

        let err = ErrorBuilder::new(stream.pos())
            .expected(self.0.clone())
            .unexpected(val)
            .build();
        Err(err)
    }
}
