use super::Lexer;
use crate::token::{Pos, Spacing, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn lex_str(&mut self, lo: Pos) {
        self.bump();
        let mut terminated = false;
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.bump();
                terminated = true;
                break;
            }
            if ch == '\\' {
                self.bump();
                if self.peek().is_some() {
                    self.bump();
                }
                continue;
            }
            if ch == '\n' || ch == '\r' {
                break;
            }
            self.bump();
        }
        self.push(TokenKind::Str { terminated }, lo, self.pos, Spacing::Alone);
    }

    pub(super) fn lex_template_str(&mut self, lo: Pos) {
        self.bump();
        let mut terminated = false;
        while let Some(ch) = self.peek() {
            if ch == '`' {
                self.bump();
                terminated = true;
                break;
            }
            if ch == '\\' {
                self.bump();
                if self.peek().is_some() {
                    self.bump();
                }
                continue;
            }
            if ch == '\n' || ch == '\r' {
                break;
            }
            self.bump();
        }
        self.push(
            TokenKind::TemplateStr { terminated },
            lo,
            self.pos,
            Spacing::Alone,
        );
    }
}
