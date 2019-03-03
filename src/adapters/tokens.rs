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
            let res = stream
                .peek()
                .ok_or_else(|| Error::new(stream.pos(), None, Expected::Token(item.clone())))?;

            if item.clone() == res {
                stream.next();
                list.push(res);
            } else {
                return Err(Error::new(
                    stream.pos(),
                    Some(res),
                    Expected::Token(item.clone()),
                ));
            }
        }

        Ok(list)
    }
}
