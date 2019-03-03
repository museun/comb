use crate::*;

pub struct Many<A: Scanner>(A);

impl<A: Scanner> Many<A> {
    pub(crate) fn new(a: A) -> Self {
        Self(a)
    }
}

impl<A: Scanner> Scanner for Many<A> {
    type Input = A::Input;
    type Output = Vec<A::Output>;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        let mut res = vec![];
        // for counting later on
        for _ in 0.. {
            let pos = stream.pos();
            match self.0.scan(stream) {
                Ok(ok) => res.push(ok),
                Err(err) => {
                    if stream.pos() != pos {
                        return Err(err);
                    }
                    break;
                }
            }
        }
        Ok(res)
    }
}
