use super::class;
use super::Lexer;
use crate::token::{Pos, Spacing, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn lex_ident(&mut self, lo: Pos) {
        self.bump();
        while self.peek().map(class::is_ident_continue) == Some(true) {
            self.bump();
        }
        self.push(TokenKind::Ident, lo, self.pos, Spacing::Alone);
    }
}
