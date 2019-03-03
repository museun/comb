use crate::*;

pub use Either::*;

/// *Left* and *Right* kinds for an [Either<L,R>](crate::Either) type. This acts as an combinator
pub enum Either<L, R>
where
    L: Scanner,
    R: Scanner<Input = L::Input, Output = L::Output>,
{
    Left(L),
    Right(R),
}

impl<L, R> Scanner for Either<L, R>
where
    L: Scanner,
    R: Scanner<Input = L::Input, Output = L::Output>,
{
    type Input = L::Input;
    type Output = L::Output;

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        match self {
            Either::Left(left) => left.scan(stream),
            Either::Right(right) => right.scan(stream),
        }
    }
}
