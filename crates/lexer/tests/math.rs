use dim_lang_lexer::{lex, NumBase, Spacing, TokenKind};

fn kinds(source: &str) -> Vec<TokenKind> {
    lex(source.to_string()).into_iter().map(|t| t.kind).collect()
}

fn kinds_no_trivia(source: &str) -> Vec<TokenKind> {
    lex(source.to_string())
        .into_iter()
        .filter(|t| !matches!(
            t.kind,
            TokenKind::Whitespace | TokenKind::Newline | TokenKind::LineComment
                | TokenKind::BlockComment { .. }
        ))
        .map(|t| t.kind)
        .collect()
}

#[test]
fn addition() {
    assert_eq!(
        kinds_no_trivia("1 + 2"),
        vec![
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Plus,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Eof,
        ]
    );
}

#[test]
fn chained_arithmetic() {
    assert_eq!(
        kinds_no_trivia("3 * 4 - 5 / 2"),
        vec![
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Star,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Minus,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Slash,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Eof,
        ]
    );
}

#[test]
fn modulo() {
    assert_eq!(
        kinds_no_trivia("7 % 3"),
        vec![
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Percent,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Eof,
        ]
    );
}

#[test]
fn parenthesized_expression() {
    assert_eq!(
        kinds_no_trivia("(1 + 2) * 3"),
        vec![
            TokenKind::LParen,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Plus,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::RParen,
            TokenKind::Star,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Eof,
        ]
    );
}

#[test]
fn decimal_literal() {
    assert_eq!(
        kinds_no_trivia("222.333"),
        vec![TokenKind::Decimal, TokenKind::Eof]
    );
}

#[test]
fn range_not_decimal() {
    let tokens = lex("1..2".to_string());
    let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).collect();
    assert_eq!(
        kinds,
        vec![
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Dot,
            TokenKind::Dot,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Eof,
        ]
    );
    assert_eq!(tokens[1].spacing, Spacing::Joint);
    assert_eq!(tokens[2].spacing, Spacing::Alone);
}

#[test]
fn hex_addition() {
    assert_eq!(
        kinds_no_trivia("0xFF + 1"),
        vec![
            TokenKind::Int {
                base: NumBase::Hex,
                empty_digits: false,
            },
            TokenKind::Plus,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Eof,
        ]
    );
}

#[test]
fn preserves_whitespace_trivia() {
    assert_eq!(
        kinds("1 + 2"),
        vec![
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Whitespace,
            TokenKind::Plus,
            TokenKind::Whitespace,
            TokenKind::Int {
                base: NumBase::Dec,
                empty_digits: false,
            },
            TokenKind::Eof,
        ]
    );
}
