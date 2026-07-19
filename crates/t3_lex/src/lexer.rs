//! This is the t3 lexer file

pub struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Lexer { src, pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn bump(&mut self) {
        self.pos += 1
    }
}
