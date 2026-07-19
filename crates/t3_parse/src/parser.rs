use t3_lexer::{Span, Token, TokenKind};
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(Span),
    Ident(Span),
}

impl Parser {
    #[must_use]
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).copied()
    }

    fn bump(&mut self) {
        self.pos += 1;
    }

    fn parse_expr(&mut self) -> Expr {
        let tok = match self.peek() {
            Some(e) => e,
            None => panic!("expected an expression"),
        };
        let expr = match tok.kind {
            TokenKind::Int => Expr::Int(tok.span),
            TokenKind::Ident => Expr::Ident(tok.span),
            other => panic!("expected an expression, found {other:?}"),
        };
        self.bump();
        expr
    }
}
