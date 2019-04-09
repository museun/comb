use crate::*;

#[derive(Debug, Clone)]
pub enum MessageKind<E: Clone> {
    Expected(ExpectedKind<E>),
    Error(crate::Error<E, E>),
    String(String),
}

impl<E: Clone> From<&str> for MessageKind<E> {
    fn from(s: &str) -> Self {
        MessageKind::String(s.to_string())
    }
}

impl<E: Clone> From<String> for MessageKind<E> {
    fn from(s: String) -> Self {
        MessageKind::String(s)
    }
}

#[derive(Debug, Clone)]
pub struct Message<A: Scanner>(A, MessageKind<A::Input>)
where
    A::Input: Clone;

impl<A: Scanner> Message<A>
where
    A::Input: Clone,
{
    pub(crate) fn new<M>(a: A, msg: M) -> Self
    where
        M: Into<MessageKind<A::Input>>,
    {
        Self(a, msg.into())
    }
}

impl<A: Scanner> Scanner for Message<A>
where
    A::Input: Clone,
{
    type Input = A::Input;
    type Output = A::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        self.0.scan(stream).map_err(|mut err| {
            match &self.1 {
                MessageKind::Expected(e) => {
                    err.expected = Some(e.clone());
                }
                MessageKind::String(s) => {
                    err.msg = Some(s.clone());
                }
                MessageKind::Error(e) => {
                    err = e.clone();
                }
            }
            err
        })
    }
}
