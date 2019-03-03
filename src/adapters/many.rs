use crate::*;

#[derive(Debug, Clone)]
pub struct Many<A: Scanner>(A, Option<usize>, Option<usize>);

impl<A: Scanner> Many<A> {
    pub(crate) fn new(a: A, x: Option<usize>, y: Option<usize>) -> Self {
        Self(a, x, y)
    }
}

impl<A: Scanner> Scanner for Many<A> {
    type Input = A::Input;
    type Output = Vec<A::Output>;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        let mut res = vec![];
        for i in 0.. {
            if let Some(y) = self.2 {
                if i > y {
                    break;
                }
            }

            let pos = stream.pos();
            match self.0.scan(stream) {
                Ok(ok) => res.push(ok),
                Err(err) => {
                    if let Some(x) = self.1 {
                        if res.len() < x {
                            return Err(err);
                        }
                    }
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
