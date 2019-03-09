use crate::*;

#[derive(Debug, Clone)]
pub struct Tokens<A: Clone + PartialEq>(Vec<A>);

impl<A: Clone + PartialEq> Tokens<A> {
    pub(crate) fn new(list: Vec<A>) -> Self {
        Self(list)
    }
}

impl<A: Clone + PartialEq> Scanner for Tokens<A> {
    type Input = A;
    type Output = Vec<A>;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        let mut list = vec![];
        for item in self.0.iter() {
            let val = crate::must_peek(stream)?;
            if item == &val {
                stream.next();
                list.push(val);
                continue;
            }
            let err = ErrorBuilder::new(stream.pos())
                .expected(item.clone())
                .unexpected(val.clone())
                .build();
            return Err(err);
        }
        Ok(list)
    }
}
