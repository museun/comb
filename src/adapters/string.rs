use crate::*;

pub struct StringToken(String, bool);
impl StringToken {
    pub fn new(s: String, case: bool) -> Self {
        Self(s, case)
    }
}

impl Scanner for StringToken {
    type Input = char;
    type Output = String; // shouldn't this be char

    fn scan(&self, stream: &mut Stream<Self::Input>) -> Res<Self> {
        let mut buf = Vec::with_capacity(self.0.len());

        let cmp = |a: char, b| {
            if self.1 {
                a == b
            } else {
                a.eq_ignore_ascii_case(&b)
            }
        };

        for ch in self.0.chars() {
            let val = crate::must_peek(stream)?;

            if cmp(val, ch) {
                buf.push(val as u8); // this is bad
                stream.next();
                continue;
            }

            return Err(ErrorBuilder::new(stream.pos())
                .expected(ch.clone())
                .unexpected(val.clone())
                .build());
        }

        // should have valid utf-8 here
        Ok(String::from_utf8(buf).unwrap())
    }
}
