use crate::*;

#[derive(Debug, Clone)]
pub struct Message<A: Scanner>(A, Expected<A::Input>);

impl<A: Scanner> Message<A> {
    pub(crate) fn new(a: A, msg: Expected<A::Input>) -> Self {
        Self(a, msg)
    }
}
impl<A: Scanner> Scanner for Message<A>
where
    A::Input: Clone,
{
    type Input = A::Input;
    type Output = A::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        self.0.scan(stream).map_err(|mut err| {
            err.expected = self.1.clone();
            err
        })
    }
}
