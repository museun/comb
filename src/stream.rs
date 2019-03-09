/// Stream wraps a sequence to be used with the combinator
pub struct Stream<T>(Vec<T>, usize);

impl<T> Stream<T> {
    /// Produce a new stream from an iterator. **note** this consumes the
    /// iterator. it should be [Fuse](std::iter::Fuse)d and finite.
    pub fn new<I>(data: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(data.into_iter().collect(), 0)
    }

    /// Retrieves the current position in the sequence
    pub fn pos(&self) -> usize {
        self.1
    }

    /// Sets the current position. ALlows for jumping
    pub fn set_pos(&mut self, pos: usize) -> Option<()> {
        if pos <= self.0.len() {
            self.1 = pos;
            Some(())
        } else {
            None
        }
    }

    /// Advance the stream by 1 element
    pub fn advance(&mut self, delta: usize) -> Option<()> {
        self.set_pos(self.pos() + delta)
    }

    /// Determines if we're at the end of the sequence
    pub fn eof(&self) -> bool {
        self.0.len() <= self.1
    }
}

impl<T: Clone> Stream<T> {
    /// Peek at the current element
    pub fn peek(&self) -> Option<T> {
        self.0.get(self.1).cloned()
    }

    /// Peek at an offset of the current element
    pub fn peek_at(&mut self, i: usize) -> Option<T> {
        self.0.get(self.1 + i).cloned()
    }
}

impl<T> Iterator for Stream<T> {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        self.advance(1)
    }
}

impl<T> std::fmt::Debug for Stream<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("stream")
            .field("pos", &self.1)
            .field("len", &self.0.len())
            .finish()
    }
}

impl From<&str> for Stream<char> {
    fn from(s: &str) -> Self {
        Stream::new(s.chars())
    }
}
