use dim_lang_lexer::Token;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TreeKind {
    ErrorTree,
    Expr,
    ExprLiteral,
    ExprParen,
    ExprBinary,
}

#[derive(Clone, Debug)]
pub enum Child {
    Token(Token),
    Tree(Tree),
}

#[derive(Clone, Debug)]
pub struct Tree {
    pub kind: TreeKind,
    pub children: Vec<Child>,
}

impl Tree {
    pub fn tree_children(&self) -> impl Iterator<Item = &Tree> {
        self.children.iter().filter_map(|child| match child {
            Child::Tree(tree) => Some(tree),
            Child::Token(_) => None,
        })
    }

    pub fn token_children(&self) -> impl Iterator<Item = &Token> {
        self.children.iter().filter_map(|child| match child {
            Child::Token(token) => Some(token),
            Child::Tree(_) => None,
        })
    }

    pub fn count_errors(&self) -> usize {
        let self_count = usize::from(self.kind == TreeKind::ErrorTree);
        self.children.iter().map(|child| match child {
            Child::Tree(tree) => tree.count_errors(),
            Child::Token(_) => 0,
        }).sum::<usize>() + self_count
    }
}
