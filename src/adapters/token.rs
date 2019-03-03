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
        let res = stream
            .peek()
            .ok_or_else(|| Error::new(stream.pos(), None, Expected::Token(self.0.clone())))?;
        if res == self.0 {
            stream.next();
            Ok(res)
        } else {
            Err(Error::new(
                stream.pos(),
                Some(res),
                Expected::Token(self.0.clone()),
            ))
        }
    }
}
