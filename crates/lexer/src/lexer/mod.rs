mod class;
mod ident;
mod number;
mod punct;
mod string;
mod trivia;

use crate::token::{Pos, Span, Spacing, Token, TokenKind};

pub(crate) struct Lexer<'a> {
    source: &'a str,
    pos: Pos,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            tokens: Vec::new(),
        }
    }

    fn lex(mut self) -> Vec<Token> {
        while !self.at_end() {
            self.lex_next();
        }
        self.push(TokenKind::Eof, self.pos, self.pos, Spacing::Alone);
        self.tokens
    }

    fn at_end(&self) -> bool {
        self.pos as usize >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source[self.pos as usize..].chars().next()
    }

    fn peek_nth(&self, n: usize) -> Option<char> {
        self.source[self.pos as usize..].chars().nth(n)
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8() as Pos;
        Some(ch)
    }

    fn push(&mut self, kind: TokenKind, lo: Pos, hi: Pos, spacing: Spacing) {
        self.tokens.push(Token {
            kind,
            span: Span { lo, hi },
            spacing,
        });
    }

    fn lex_next(&mut self) {
        let lo = self.pos;
        let ch = self.peek().unwrap();

        if ch.is_whitespace() {
            if ch == '\n' || ch == '\r' {
                self.lex_newline(lo);
            } else {
                self.lex_whitespace(lo);
            }
            return;
        }

        if ch == '/' {
            if self.peek_nth(1) == Some('/') {
                self.lex_line_comment(lo);
                return;
            }
            if self.peek_nth(1) == Some('*') {
                self.lex_block_comment(lo);
                return;
            }
        }

        if ch.is_ascii_digit() {
            self.lex_number(lo);
            return;
        }

        if ch == '.' && self.peek_nth(1).map(|c| c.is_ascii_digit()) == Some(true) {
            self.lex_number(lo);
            return;
        }

        if class::is_ident_start(ch) {
            self.lex_ident(lo);
            return;
        }

        if ch == '"' {
            self.lex_str(lo);
            return;
        }

        if ch == '`' {
            self.lex_template_str(lo);
            return;
        }

        self.lex_punct(lo);
    }
}

pub fn lex(source: &str) -> Vec<Token> {
    Lexer::new(source).lex()
}
