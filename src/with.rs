use crate::*;

#[derive(Debug, Clone)]
pub struct With<A, B>(A, B)
where
    A: Scanner,
    B: Scanner<Input = A::Input>;

impl<A, B> With<A, B>
where
    A: Scanner,
    B: Scanner<Input = A::Input>,
{
    pub(crate) fn new(a: A, b: B) -> Self {
        Self(a, b)
    }
}

impl<A, B> Scanner for With<A, B>
where
    A: Scanner,
    B: Scanner<Input = A::Input>,
{
    type Input = A::Input;
    type Output = B::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        self.0.scan(stream)?;
        self.1.scan(stream)
    }
}
