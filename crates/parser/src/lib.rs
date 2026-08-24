mod grammar;
mod parser;
mod tree;

pub use dim_lang_lexer::{Span, Token, TokenKind};
pub use tree::{Child, Tree, TreeKind};

use dim_lang_lexer::{self, Token as LexerToken};

use grammar::parse_expr as parse_expr_grammar;
use parser::Parser;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn lexer_version() -> &'static str {
    dim_lang_lexer::version()
}

pub fn parse_expr(source: &str) -> Tree {
    let tokens = without_trivia(dim_lang_lexer::lex(source.to_string()));
    let tokens = without_eof(tokens);
    let mut parser = Parser::new(tokens);
    parse_expr_grammar(&mut parser);
    parser.build_tree()
}

fn without_trivia(tokens: Vec<LexerToken>) -> Vec<LexerToken> {
    tokens
        .into_iter()
        .filter(|token| {
            !matches!(
                token.kind,
                TokenKind::Whitespace
                    | TokenKind::Newline
                    | TokenKind::LineComment
                    | TokenKind::BlockComment { .. }
            )
        })
        .collect()
}

fn without_eof(mut tokens: Vec<LexerToken>) -> Vec<LexerToken> {
    if tokens.last().is_some_and(|token| token.kind == TokenKind::Eof) {
        tokens.pop();
    }
    tokens
}
