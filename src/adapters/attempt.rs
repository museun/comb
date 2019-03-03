use crate::*;

#[derive(Debug, Clone)]
pub struct Attempt<A: Scanner>(A);

impl<A: Scanner> Attempt<A> {
    pub(crate) fn new(a: A) -> Self {
        Self(a)
    }
}

impl<A: Scanner> Scanner for Attempt<A> {
    type Input = A::Input;
    type Output = A::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        let pos = stream.pos();
        let res = self.0.scan(stream);
        if res.is_err() {
            stream.set_pos(pos);
        }
        res
    }
}
