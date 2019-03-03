use crate::*;

#[derive(Debug, Clone)]
pub struct Or<A, B>(A, B)
where
    A: Scanner,
    B: Scanner<Input = A::Input, Output = A::Output>;

impl<A, B> Or<A, B>
where
    A: Scanner,
    B: Scanner<Input = A::Input, Output = A::Output>,
{
    pub(crate) fn new(a: A, b: B) -> Self {
        Self(a, b)
    }
}

impl<A, B> Scanner for Or<A, B>
where
    A: Scanner,
    B: Scanner<Input = A::Input, Output = A::Output>,
{
    type Input = A::Input;
    type Output = B::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        let pos = stream.pos();
        match self.0.scan(stream) {
            Err(..) if pos == stream.pos() => self.1.scan(stream),
            other => other,
        }
    }
}
