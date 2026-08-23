use super::Lexer;
use crate::token::{Pos, Spacing, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn lex_whitespace(&mut self, lo: Pos) {
        while let Some(ch) = self.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            if !ch.is_whitespace() {
                break;
            }
            self.bump();
        }
        self.push(TokenKind::Whitespace, lo, self.pos, Spacing::Alone);
    }

    pub(super) fn lex_newline(&mut self, lo: Pos) {
        if self.peek() == Some('\r') {
            self.bump();
            if self.peek() == Some('\n') {
                self.bump();
            }
        } else {
            self.bump();
        }
        while self.peek() == Some('\n') || self.peek() == Some('\r') {
            if self.peek() == Some('\r') {
                self.bump();
                if self.peek() == Some('\n') {
                    self.bump();
                }
            } else {
                self.bump();
            }
        }
        self.push(TokenKind::Newline, lo, self.pos, Spacing::Alone);
    }

    pub(super) fn lex_line_comment(&mut self, lo: Pos) {
        self.bump();
        self.bump();
        while let Some(ch) = self.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            self.bump();
        }
        self.push(TokenKind::LineComment, lo, self.pos, Spacing::Alone);
    }

    pub(super) fn lex_block_comment(&mut self, lo: Pos) {
        self.bump();
        self.bump();
        let mut level = 1u32;
        while !self.at_end() {
            match (self.peek(), self.peek_nth(1)) {
                (Some('/'), Some('*')) => {
                    self.bump();
                    self.bump();
                    level += 1;
                }
                (Some('*'), Some('/')) => {
                    self.bump();
                    self.bump();
                    level -= 1;
                    if level == 0 {
                        self.push(
                            TokenKind::BlockComment {
                                terminated: true,
                                level: 1,
                            },
                            lo,
                            self.pos,
                            Spacing::Alone,
                        );
                        return;
                    }
                }
                _ => {
                    self.bump();
                }
            }
        }
        self.push(
            TokenKind::BlockComment {
                terminated: false,
                level,
            },
            lo,
            self.pos,
            Spacing::Alone,
        );
    }
}
