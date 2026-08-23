mod kind;
mod span;

pub use kind::{NumBase, TokenKind};
pub use span::{Pos, Span, Spacing};

use span::{Span as TokenSpan, Spacing as TokenSpacing};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TokenSpan,
    pub spacing: TokenSpacing,
}
