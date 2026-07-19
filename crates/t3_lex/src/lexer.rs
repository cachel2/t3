//! This is the t3 lexer file

use crate::{
    TokenKind::Var,
    token::{Span, Token, TokenKind},
};

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
        self.pos += 1;
    }

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
                while b.is_ascii_digit() {
                    self.bump();
                }
                TokenKind::Int
            }

            Some(other) => panic!("did not expect that byte {}", other as char),
        };

        let end = self.pos;
        Token {
            kind,
            span: Span {
                start: start as u32,
                end: end as u32,
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
        assert_eq!(lx.next_token().kind, TokenKind::Ident);
    }
}
