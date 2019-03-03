pub struct Stream<T>(Vec<T>, usize);

impl<T> Stream<T> {
    pub fn new<I>(data: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(data.into_iter().collect(), 0)
    }

    pub fn pos(&self) -> usize {
        self.1
    }

    pub fn set_pos(&mut self, pos: usize) -> Option<()> {
        if pos <= self.0.len() {
            self.1 = pos;
            Some(())
        } else {
            None
        }
    }

    pub fn advance(&mut self, delta: usize) -> Option<()> {
        self.set_pos(self.pos() + delta)
    }

    pub fn eof(&self) -> bool {
        self.0.len() <= self.1
    }
}

impl<T: Clone> Stream<T> {
    pub fn peek(&self) -> Option<T> {
        self.0.get(self.1).cloned()
    }

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
