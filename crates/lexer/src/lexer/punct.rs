use super::class;
use super::Lexer;
use crate::token::{Pos, Spacing, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn lex_punct(&mut self, lo: Pos) {
        let ch = self.bump().unwrap();

        match ch {
            '(' => self.push(TokenKind::LParen, lo, self.pos, Spacing::Alone),
            ')' => self.push(TokenKind::RParen, lo, self.pos, Spacing::Alone),
            '{' => self.push(TokenKind::LBrace, lo, self.pos, Spacing::Alone),
            '}' => self.push(TokenKind::RBrace, lo, self.pos, Spacing::Alone),
            '[' => self.push(TokenKind::LBracket, lo, self.pos, Spacing::Alone),
            ']' => self.push(TokenKind::RBracket, lo, self.pos, Spacing::Alone),
            '+' => self.push(TokenKind::Plus, lo, self.pos, Spacing::Alone),
            '*' => self.push(TokenKind::Star, lo, self.pos, Spacing::Alone),
            '%' => self.push(TokenKind::Percent, lo, self.pos, Spacing::Alone),
            '^' => self.push(TokenKind::Caret, lo, self.pos, Spacing::Alone),
            '~' => self.push(TokenKind::Tilde, lo, self.pos, Spacing::Alone),
            ',' => self.push(TokenKind::Comma, lo, self.pos, Spacing::Alone),
            ';' => self.push(TokenKind::Semi, lo, self.pos, Spacing::Alone),
            '@' => self.push(TokenKind::At, lo, self.pos, Spacing::Alone),
            '#' => self.push(TokenKind::Pound, lo, self.pos, Spacing::Alone),
            '$' => self.push(TokenKind::Dollar, lo, self.pos, Spacing::Alone),
            '?' => self.push(TokenKind::Question, lo, self.pos, Spacing::Alone),
            '\\' => self.push(TokenKind::Backslash, lo, self.pos, Spacing::Alone),
            '/' => self.push(TokenKind::Slash, lo, self.pos, Spacing::Alone),
            '=' => self.lex_eq(lo),
            '-' => self.lex_minus(lo),
            '<' => self.lex_lt(lo),
            '>' => self.lex_gt(lo),
            '&' => self.lex_amp(lo),
            '|' => self.lex_pipe(lo),
            ':' => self.lex_colon(lo),
            '.' => self.lex_dot(lo),
            '!' => self.lex_bang(lo),
            _ if class::is_unicode_punct(ch) => {
                self.push(TokenKind::Punct(ch), lo, self.pos, Spacing::Alone)
            }
            _ => self.push(TokenKind::Unknown(ch), lo, self.pos, Spacing::Alone),
        }
    }

    fn push_joint_pair(&mut self, first: TokenKind, second: TokenKind, lo: Pos) {
        self.push(first, lo, self.pos, Spacing::Joint);
        let second_lo = self.pos;
        self.bump();
        self.push(second, second_lo, self.pos, Spacing::Alone);
    }

    fn lex_eq(&mut self, lo: Pos) {
        if self.peek() == Some('=') {
            self.push_joint_pair(TokenKind::Eq, TokenKind::Eq, lo);
        } else {
            self.push(TokenKind::Eq, lo, self.pos, Spacing::Alone);
        }
    }

    fn lex_minus(&mut self, lo: Pos) {
        if self.peek() == Some('>') {
            self.push_joint_pair(TokenKind::Minus, TokenKind::Gt, lo);
        } else {
            self.push(TokenKind::Minus, lo, self.pos, Spacing::Alone);
        }
    }

    fn lex_lt(&mut self, lo: Pos) {
        match self.peek() {
            Some('=') => self.push_joint_pair(TokenKind::Lt, TokenKind::Eq, lo),
            Some('<') => self.push_joint_pair(TokenKind::Lt, TokenKind::Lt, lo),
            _ => self.push(TokenKind::Lt, lo, self.pos, Spacing::Alone),
        }
    }

    fn lex_gt(&mut self, lo: Pos) {
        if self.peek() == Some('=') {
            self.push_joint_pair(TokenKind::Gt, TokenKind::Eq, lo);
        } else {
            self.push(TokenKind::Gt, lo, self.pos, Spacing::Alone);
        }
    }

    fn lex_amp(&mut self, lo: Pos) {
        if self.peek() == Some('&') {
            self.push_joint_pair(TokenKind::Amp, TokenKind::Amp, lo);
        } else {
            self.push(TokenKind::Amp, lo, self.pos, Spacing::Alone);
        }
    }

    fn lex_pipe(&mut self, lo: Pos) {
        if self.peek() == Some('|') {
            self.push_joint_pair(TokenKind::Pipe, TokenKind::Pipe, lo);
        } else {
            self.push(TokenKind::Pipe, lo, self.pos, Spacing::Alone);
        }
    }

    fn lex_colon(&mut self, lo: Pos) {
        match self.peek() {
            Some(':') => self.push_joint_pair(TokenKind::Colon, TokenKind::Colon, lo),
            Some('=') => self.push_joint_pair(TokenKind::Colon, TokenKind::Eq, lo),
            _ => self.push(TokenKind::Colon, lo, self.pos, Spacing::Alone),
        }
    }

    fn lex_dot(&mut self, lo: Pos) {
        if self.peek() == Some('.') {
            self.push_joint_pair(TokenKind::Dot, TokenKind::Dot, lo);
        } else {
            self.push(TokenKind::Dot, lo, self.pos, Spacing::Alone);
        }
    }

    fn lex_bang(&mut self, lo: Pos) {
        if self.peek() == Some('=') {
            self.push_joint_pair(TokenKind::Bang, TokenKind::Eq, lo);
        } else {
            self.push(TokenKind::Bang, lo, self.pos, Spacing::Alone);
        }
    }
}
