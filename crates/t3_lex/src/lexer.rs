//! This is the t3 lexer file

use crate::token::{Span, Token, TokenKind};
/// Holds the source bytes and a \pos` cursor into them.
pub struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// `new()` takes the source and builds an [`Lexer`] type structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use t3_lexer::Lexer;
    /// let mut lx = Lexer::new(b"i32 main() { var i32 x = 10; return x; }");
    /// ```
    #[must_use]
    pub fn new(src: &'a [u8]) -> Self {
        Lexer { src, pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn bump(&mut self) {
        self.pos += 1;
    }

    /// Scans and returns the next token, skipping any whitespace.
    ///
    /// One the source is exhausted it returns [`TokenKind::Eof`], and
    /// keeps returning it on every further call.
    ///
    /// # Examples
    ///
    /// ```
    /// use t3_lexer::{Lexer, TokenKind};
    /// let mut lx = Lexer::new(b"x = 10");
    /// assert_eq!(lx.next_token().kind, TokenKind::Ident);
    /// ```
    ///
    /// # Panics
    ///
    /// Currently panics on every byte that isn't whitespace, an identifier
    /// character, a digit, or one of `(){};=`. This is temporary, will emit
    /// a diagnostic later.
    pub fn next_token(&mut self) -> Token {
        while let Some(b) = self.peek() {
            if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
                self.bump();
            } else {
                break;
            }
        }

        let start = self.pos;

        let kind = match self.peek() {
            None => TokenKind::Eof,
            Some(b'(') => {
                self.bump();
                TokenKind::LParen
            }
            Some(b')') => {
                self.bump();
                TokenKind::RParen
            }
            Some(b'}') => {
                self.bump();
                TokenKind::RBrace
            }
            Some(b'{') => {
                self.bump();
                TokenKind::LBrace
            }
            Some(b';') => {
                self.bump();
                TokenKind::Semi
            }
            Some(b'=') => {
                self.bump();
                TokenKind::Eq
            }

            Some(b) if b.is_ascii_alphabetic() || b == b'_' => {
                while let Some(b) = self.peek() {
                    if b.is_ascii_alphanumeric() || b == b'_' {
                        self.bump();
                    } else {
                        break;
                    }
                }
                let text = &self.src[start..self.pos];
                match text {
                    b"var" => TokenKind::Var,
                    b"return" => TokenKind::Return,
                    _ => TokenKind::Ident,
                }
            }

            Some(b) if b.is_ascii_digit() => {
                while let Some(b) = self.peek() {
                    if b.is_ascii_digit() {
                        self.bump();
                    } else {
                        break;
                    }
                }
                TokenKind::Int
            }
            Some(other) => panic!("did not expect that byte {}", other as char),
        };

        let end = self.pos;
        Token {
            kind,
            span: Span {
                start: u32::try_from(start).expect("Error: truncation problem with 32/64 bits"),
                end: u32::try_from(end).expect("Error: truncation problem with 32/64 bits"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_milestone() {
        let mut lx = Lexer::new(b"i32 main() { var i32 x = 10; return x; }");

        let mut kinds = Vec::new();
        loop {
            let k = lx.next_token().kind;
            kinds.push(k);
            if k == TokenKind::Eof {
                break;
            }
        }
        assert_eq!(
            kinds,
            vec![
                TokenKind::Ident, // i32
                TokenKind::Ident, // main
                TokenKind::LParen,
                TokenKind::RParen,
                TokenKind::LBrace,
                TokenKind::Var,
                TokenKind::Ident,
                TokenKind::Ident,
                TokenKind::Eq,
                TokenKind::Int,
                TokenKind::Semi,
                TokenKind::Return,
                TokenKind::Ident,
                TokenKind::Semi,
                TokenKind::RBrace,
                TokenKind::Eof,
            ]
        );
    }
}
