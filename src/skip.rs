use crate::*;

pub struct Skip<A, B>(A, B)
where
    A: Scanner,
    B: Scanner<Input = A::Input>;

impl<A, B> Skip<A, B>
where
    A: Scanner,
    B: Scanner<Input = A::Input>,
{
    pub(crate) fn new(a: A, b: B) -> Self {
        Self(a, b)
    }
}

impl<A, B> Scanner for Skip<A, B>
where
    A: Scanner,
    B: Scanner<Input = A::Input>,
{
    type Input = A::Input;
    type Output = A::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        let res = self.0.scan(stream)?;
        self.1.scan(stream)?;
        Ok(res)
    }
}
