use dim_lang_lexer::TokenKind;

use crate::parser::{MarkClosed, Parser};
use crate::tree::TreeKind;

pub fn parse_expr(p: &mut Parser) {
    let mark = p.open();
    expr_rec(p, TokenKind::Eof);
    p.close(mark, TreeKind::Expr);
}

fn expr_rec(p: &mut Parser, left: TokenKind) {
    let mut lhs = expr_delimited(p);

    loop {
        let right = p.nth(0);
        if right_binds_tighter(left, right) {
            let mark = p.open_before(lhs);
            p.advance();
            expr_rec(p, right);
            lhs = p.close(mark, TreeKind::ExprBinary);
        } else {
            break;
        }
    }
}

fn expr_delimited(p: &mut Parser) -> MarkClosed {
    let mark = p.open();

    let kind = match p.nth(0) {
        TokenKind::Int { .. } | TokenKind::Decimal => {
            p.advance();
            TreeKind::ExprLiteral
        }
        TokenKind::LParen => {
            p.expect(TokenKind::LParen);
            expr_rec(p, TokenKind::Eof);
            p.expect(TokenKind::RParen);
            TreeKind::ExprParen
        }
        _ => {
            if !p.eof() {
                p.advance();
            }
            TreeKind::ErrorTree
        }
    };

    p.close(mark, kind)
}

fn right_binds_tighter(left: TokenKind, right: TokenKind) -> bool {
    fn tightness(kind: TokenKind) -> Option<usize> {
        [
            [TokenKind::Plus, TokenKind::Minus].as_slice(),
            &[TokenKind::Star, TokenKind::Slash, TokenKind::Percent],
        ]
        .iter()
        .position(|level| level.contains(&kind))
    }

    let Some(right_tightness) = tightness(right) else {
        return false;
    };
    let Some(left_tightness) = tightness(left) else {
        assert!(left == TokenKind::Eof);
        return true;
    };

    right_tightness > left_tightness
}
