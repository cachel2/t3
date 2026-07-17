//! Tokens: the lexical units of t3
//!
//! `TokenKind` is the category itself, 'Token' matches it with `Span`

/// `TokenKind` enum is the 'identifier' or class from the tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// Identifier or contextual word, ej. `main` or `i32`
    Ident,
    /// literal integer
    Int,
    /// Keyword `var`
    Var,
    /// Keyword `return`
    Return,
    /// Left parenthesis `(`
    LParen,
    /// Right parenthesis `)`
    RParen,
    /// Left brace `{`
    LBrace,
    /// Right brace `}`
    RBrace,
    /// Semicolon `;`
    Semi,
    /// Equals sign `=`
    Eq,
    /// End of the file
    Eof,
}

/// A byte range in the source: start..end
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The start of the span
    pub start: u32,
    /// The end of the span
    pub end: u32,
}

/// This is a lexical token, it just stores the class
/// and where does it lives in the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// Which class of token it is.
    pub kind: TokenKind,
    /// The range defined by the start and the offset of bytes
    /// in the source text.
    pub span: Span,
}
