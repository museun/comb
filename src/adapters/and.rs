use crate::*;

#[derive(Debug, Clone)]
pub struct And<A, B>(A, B)
where
    A: Scanner,
    B: Scanner<Input = A::Input>;

impl<A, B> And<A, B>
where
    A: Scanner,
    B: Scanner<Input = A::Input>,
{
    pub(crate) fn new(a: A, b: B) -> Self {
        Self(a, b)
    }
}

impl<A, B> Scanner for And<A, B>
where
    A: Scanner,
    B: Scanner<Input = A::Input>,
{
    type Input = A::Input;
    type Output = (A::Output, B::Output);

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        Ok((self.0.scan(stream)?, self.1.scan(stream)?))
    }
}
