use crate::*;

#[derive(Debug, Clone)]
pub struct Optional<A: Scanner>(A);

impl<A: Scanner> Optional<A> {
    pub(crate) fn new(a: A) -> Self {
        Self(a)
    }
}

impl<A: Scanner> Scanner for Optional<A> {
    type Input = A::Input;
    type Output = Option<A::Output>;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        let pos = stream.pos();
        match self.0.scan(stream) {
            Err(..) if pos == stream.pos() => Ok(None),
            Err(err) => Err(err),
            Ok(ok) => Ok(Some(ok)),
        }
    }
}
