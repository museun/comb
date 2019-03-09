macro_rules! export {
    ($($e:tt),* $(,)?) => {
        $(
            mod $e;
            pub use self::$e::*;
        )*
    };
}

export!(
    and,      //
    any,      //
    attempt,  //
    either,   //
    end,      //
    expect,   //
    fail,     //
    many,     //
    map,      //
    message,  //
    optional, //
    or,       //
    range,    //
    scanwith, //
    skip,     //
    string,   //
    then,     //
    token,    //
    tokens,   //
    value,    //
    with,     //
);

pub(super) fn must_peek<E: Clone>(stream: &mut crate::Stream<E>) -> Result<E, crate::Error<E, E>> {
    stream.peek().ok_or_else(|| crate::Error {
        pos: stream.pos(),
        unexpected: Some(crate::ExpectedKind::End),
        expected: Some(crate::ExpectedKind::Any),
        msg: None,
    })
}
