use crate::*;

#[derive(Debug, Clone)]
pub struct Many<A: Scanner>
where
    A::Input: std::fmt::Debug,
    A::Output: std::fmt::Debug,
{
    scan: A,
    x: Option<usize>,
    y: Option<usize>,
}

impl<A: Scanner> Many<A>
where
    A::Input: std::fmt::Debug,
    A::Output: std::fmt::Debug,
{
    pub(crate) fn new(scan: A, x: Option<usize>, y: Option<usize>) -> Self {
        Self { scan, x, y }
    }
}

impl<A: Scanner> Scanner for Many<A>
where
    A::Input: std::fmt::Debug,
    A::Output: std::fmt::Debug,
{
    type Input = A::Input;
    type Output = Vec<A::Output>;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        let mut res = vec![];
        for i in 0.. {
            if let Some(y) = self.y {
                if i > y {
                    break;
                }
            }

            let pos = stream.pos();
            match self.scan.scan(stream) {
                Ok(ok) => res.push(ok),
                Err(err) => {
                    if let Some(x) = self.x {
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
