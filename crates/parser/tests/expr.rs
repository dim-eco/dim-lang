use dim_lang_lexer::{NumBase, TokenKind};

use dim_lang_parser::{parse_expr, Child, Tree, TreeKind};

fn tree_children(tree: &Tree) -> Vec<&Tree> {
    tree.tree_children().collect()
}

fn token_kinds(tree: &Tree) -> Vec<TokenKind> {
    tree.token_children().map(|token| token.kind).collect()
}

fn binary_op(tree: &Tree) -> Option<TokenKind> {
    if tree.kind != TreeKind::ExprBinary {
        return None;
    }
    tree.token_children().next().map(|token| token.kind)
}

#[test]
fn addition() {
    let tree = parse_expr("1 + 2");
    assert_eq!(tree.kind, TreeKind::Expr);

    let children = tree_children(&tree);
    assert_eq!(children.len(), 1);

    let binary = children[0];
    assert_eq!(binary.kind, TreeKind::ExprBinary);
    assert_eq!(binary_op(binary), Some(TokenKind::Plus));

    let operands = tree_children(binary);
    assert_eq!(operands.len(), 2);
    assert_eq!(operands[0].kind, TreeKind::ExprLiteral);
    assert_eq!(operands[1].kind, TreeKind::ExprLiteral);
    assert_eq!(
        token_kinds(operands[0]),
        vec![TokenKind::Int {
            base: NumBase::Dec,
            empty_digits: false,
        }]
    );
    assert_eq!(
        token_kinds(operands[1]),
        vec![TokenKind::Int {
            base: NumBase::Dec,
            empty_digits: false,
        }]
    );
}

#[test]
fn precedence() {
    let tree = parse_expr("1 + 2 * 3");
    let binary = tree_children(&tree)[0];
    assert_eq!(binary.kind, TreeKind::ExprBinary);
    assert_eq!(binary_op(binary), Some(TokenKind::Plus));

    let operands = tree_children(binary);
    assert_eq!(operands[0].kind, TreeKind::ExprLiteral);

    let mul = operands[1];
    assert_eq!(mul.kind, TreeKind::ExprBinary);
    assert_eq!(binary_op(mul), Some(TokenKind::Star));
}

#[test]
fn left_associativity() {
    let tree = parse_expr("10 - 3 - 2");
    let outer = tree_children(&tree)[0];
    assert_eq!(binary_op(outer), Some(TokenKind::Minus));

    let operands = tree_children(outer);
    assert_eq!(operands[0].kind, TreeKind::ExprBinary);
    assert_eq!(operands[1].kind, TreeKind::ExprLiteral);

    let inner = operands[0];
    assert_eq!(binary_op(inner), Some(TokenKind::Minus));
    assert_eq!(tree_children(inner)[0].kind, TreeKind::ExprLiteral);
    assert_eq!(tree_children(inner)[1].kind, TreeKind::ExprLiteral);
}

#[test]
fn parenthesized_expression() {
    let tree = parse_expr("(1 + 2) * 3");
    let binary = tree_children(&tree)[0];
    assert_eq!(binary_op(binary), Some(TokenKind::Star));

    let operands = tree_children(binary);
    assert_eq!(operands[0].kind, TreeKind::ExprParen);
    assert_eq!(operands[1].kind, TreeKind::ExprLiteral);
}

#[test]
fn chained_arithmetic() {
    let tree = parse_expr("3 * 4 - 5 / 2");
    assert_eq!(tree.count_errors(), 0);
    assert_eq!(tree.kind, TreeKind::Expr);
}

#[test]
fn modulo() {
    let tree = parse_expr("7 % 3");
    let binary = tree_children(&tree)[0];
    assert_eq!(binary.kind, TreeKind::ExprBinary);
    assert_eq!(binary_op(binary), Some(TokenKind::Percent));
}

#[test]
fn decimal_literal() {
    let tree = parse_expr("222.333");
    let literal = tree_children(&tree)[0];
    assert_eq!(literal.kind, TreeKind::ExprLiteral);
    assert_eq!(token_kinds(literal), vec![TokenKind::Decimal]);
}

#[test]
fn whitespace_trivia() {
    let plain = parse_expr("1+2");
    let spaced = parse_expr("1 + 2");
    assert_eq!(plain.count_errors(), 0);
    assert_eq!(spaced.count_errors(), 0);
    assert_eq!(
        tree_children(&plain)[0].kind,
        tree_children(&spaced)[0].kind
    );
    assert_eq!(
        binary_op(tree_children(&plain)[0]),
        binary_op(tree_children(&spaced)[0])
    );
}

#[test]
fn incomplete_binary() {
    let tree = parse_expr("1 +");
    assert!(tree.count_errors() > 0);

    let binary = tree_children(&tree)[0];
    assert_eq!(binary.kind, TreeKind::ExprBinary);
    assert_eq!(binary_op(binary), Some(TokenKind::Plus));

    let operands = tree_children(binary);
    assert_eq!(operands[0].kind, TreeKind::ExprLiteral);
    assert_eq!(operands[1].kind, TreeKind::ErrorTree);
}

#[test]
fn unclosed_paren() {
    let tree = parse_expr("(1 + 2");
    assert!(tree.count_errors() > 0);

    let paren = tree_children(&tree)[0];
    assert_eq!(paren.kind, TreeKind::ExprParen);

    let has_error = paren.children.iter().any(|child| match child {
        Child::Tree(inner) => inner.kind == TreeKind::ErrorTree,
        Child::Token(_) => false,
    });
    assert!(has_error);
}

#[test]
fn unexpected_token() {
    let tree = parse_expr("1 + )");
    assert!(tree.count_errors() > 0);

    let binary = tree_children(&tree)[0];
    assert_eq!(binary.kind, TreeKind::ExprBinary);
    assert_eq!(binary_op(binary), Some(TokenKind::Plus));
}
