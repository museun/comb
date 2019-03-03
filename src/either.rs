use crate::*;

pub use Either::*;
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
    fn scan(&self, stream: &mut Stream<Self::Input>) -> ScannerResult<Self::Output, Self::Input> {
        match self {
            Either::Left(left) => left.scan(stream),
            Either::Right(right) => right.scan(stream),
        }
    }
}
