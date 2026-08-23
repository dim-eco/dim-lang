use super::Lexer;
use crate::token::{NumBase, Pos, Spacing, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn lex_number(&mut self, lo: Pos) {
        if self.peek() == Some('0') {
            match self.peek_nth(1) {
                Some('x' | 'X') => {
                    self.bump();
                    self.bump();
                    let digits_lo = self.pos;
                    while self.peek().map(|c| c.is_ascii_hexdigit()) == Some(true) {
                        self.bump();
                    }
                    let empty_digits = self.pos == digits_lo;
                    self.push(
                        TokenKind::Int {
                            base: NumBase::Hex,
                            empty_digits,
                        },
                        lo,
                        self.pos,
                        Spacing::Alone,
                    );
                    return;
                }
                Some('o' | 'O') => {
                    self.bump();
                    self.bump();
                    let digits_lo = self.pos;
                    while self.peek().map(|c| c == '0' || ('1'..='7').contains(&c)) == Some(true) {
                        self.bump();
                    }
                    let empty_digits = self.pos == digits_lo;
                    self.push(
                        TokenKind::Int {
                            base: NumBase::Oct,
                            empty_digits,
                        },
                        lo,
                        self.pos,
                        Spacing::Alone,
                    );
                    return;
                }
                Some('b' | 'B') => {
                    self.bump();
                    self.bump();
                    let digits_lo = self.pos;
                    while self.peek().map(|c| c == '0' || c == '1') == Some(true) {
                        self.bump();
                    }
                    let empty_digits = self.pos == digits_lo;
                    self.push(
                        TokenKind::Int {
                            base: NumBase::Bin,
                            empty_digits,
                        },
                        lo,
                        self.pos,
                        Spacing::Alone,
                    );
                    return;
                }
                _ => {}
            }
        }

        while self.peek().map(|c| c.is_ascii_digit()) == Some(true) {
            self.bump();
        }

        if self.peek() == Some('.') && self.peek_nth(1) != Some('.') {
            let after_dot = self.peek_nth(1);
            if after_dot.map(|c| c.is_ascii_digit()) == Some(true) {
                self.bump();
                while self.peek().map(|c| c.is_ascii_digit()) == Some(true) {
                    self.bump();
                }
                self.push(TokenKind::Decimal, lo, self.pos, Spacing::Alone);
                return;
            }
        }

        self.push(
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            lo,
            self.pos,
            Spacing::Alone,
        );
    }
}
