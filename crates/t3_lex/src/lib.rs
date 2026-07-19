//! This is the t3 lexer
//!
//! In this crate implements the lexical analysis.
pub mod lexer;
pub mod token;
//TODO: expose api's from this crate.

pub use lexer::Lexer;
pub use token::{Span, Token, TokenKind};
