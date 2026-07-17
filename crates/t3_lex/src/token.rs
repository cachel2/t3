#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Ident,
    Int,
    Var,
    Return,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semi,
    Eq,
    Eof,
}
