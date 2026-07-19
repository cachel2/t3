//! This is the t3 parser
//!
//! In this crate implements the parser for the compiler.
//! The parser depends on the lexer crate for `Token`, `TokenKind`, `Span` and `Lexer`.
/// Parser implementation.
pub mod parser;

pub use parser::{Expr, Parser};
