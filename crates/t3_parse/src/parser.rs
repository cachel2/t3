use inkwell::values::PointerValue;
use std::collections::HashMap;
use t3_lexer::{
    Span, Token,
    TokenKind::{self, Eq, Ident, Int, Return, Semi, Var},
};

pub struct Codegen<'src, 'ctx> {
    vars: HashMap<&'src [u8], PointerValue<'ctx>>,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(Span),
    Ident(Span),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VarDecl { ty: Span, name: Span, init: Expr },
    Return { expr: Expr },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    pub name: Span,
    pub return_ty: Span,
    pub body: Vec<Stmt>,
}

impl Parser {
    #[must_use]
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.pos).copied()
    }

    /// The token at the cursor. Panics at end of input.
    fn current(&self) -> Token {
        self.peek().expect("unexpected end of input")
    }

    /// Consumes and returns the token at the cursor.
    fn bump(&mut self) -> Token {
        let tok = self.current();
        self.pos += 1;
        tok
    }

    /// Consumes the token at the cursor, requiring it to be `kind`.
    fn expect(&mut self, kind: TokenKind) -> Span {
        let tok = self.bump();
        assert!(tok.kind == kind, "expected {kind:?}, found {:?}", tok.kind);
        tok.span
    }

    fn parse_expr(&mut self) -> Expr {
        let tok = self.bump();
        match tok.kind {
            Int => Expr::Int(tok.span),
            Ident => Expr::Ident(tok.span),
            other => panic!("expected an expression, found {other:?}"),
        }
    }

    fn parse_stmt(&mut self) -> Stmt {
        let tok = self.bump();
        match tok.kind {
            Var => {
                let ty = self.expect(Ident);
                let name = self.expect(Ident);
                self.expect(Eq);
                let init = self.parse_expr();
                self.expect(Semi);
                Stmt::VarDecl { ty, name, init }
            }
            Return => {
                let expr = self.parse_expr();
                self.expect(Semi);
                Stmt::Return { expr }
            }
            other => panic!("expected a statement, found {other:?}"),
        }
    }

    fn parse_func(&mut self) -> Func {
        let return_ty = self.expect(TokenKind::Ident);

        let name = self.expect(Ident);

        self.expect(TokenKind::LParen);
        self.expect(TokenKind::RParen);

        self.expect(TokenKind::LBrace);
        let mut body = Vec::new();
        while self.current().kind != TokenKind::RBrace {
            body.push(self.parse_stmt());
        }

        self.expect(TokenKind::RBrace);
        Func {
            return_ty,
            name,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use t3_lexer::{Lexer, TokenKind::LParen};

    fn lex(src: &[u8]) -> Vec<Token> {
        let mut lx = Lexer::new(src);
        let mut tokens = Vec::new();
        loop {
            let t = lx.next_token();
            let eof = t.kind == TokenKind::Eof;
            tokens.push(t);
            if eof {
                break;
            }
        }
        tokens
    }

    #[test]
    fn parse_var_decl() {
        let mut p = Parser::new(lex(b"var i32 x = 10;"));
        let stmt = p.parse_stmt();
        assert!(matches!(
            stmt,
            Stmt::VarDecl {
                init: Expr::Int(_),
                ..
            }
        ));
    }

    #[test]
    fn parse_func() {
        let mut fun = Parser::new(lex(b"i32 main() { var i32 x = 10; return x; }"));
        let parsed = fun.parse_func();

        assert_eq!(parsed.body.len(), 2);
        assert!(matches!(
            parsed.body[0],
            Stmt::VarDecl {
                init: Expr::Int(_),
                ..
            }
        ));

        assert!(matches!(
            parsed.body[1],
            Stmt::Return {
                expr: Expr::Ident(_),
            }
        ));
    }
}
