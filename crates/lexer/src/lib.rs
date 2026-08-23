mod lexer;
mod token;

pub use token::{NumBase, Pos, Span, Spacing, Token, TokenKind};

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn lex(string: String) -> Vec<Token> {
    lexer::lex(&string)
}
