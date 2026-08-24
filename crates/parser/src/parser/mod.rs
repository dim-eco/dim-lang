use std::cell::Cell;

use dim_lang_lexer::{Token, TokenKind};

use crate::tree::{Child, Tree, TreeKind};

#[derive(Clone, Copy, Debug)]
pub struct MarkOpened {
    index: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct MarkClosed {
    index: usize,
}

#[derive(Debug)]
enum Event {
    Open { kind: TreeKind },
    Close,
    Advance,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    fuel: Cell<u32>,
    events: Vec<Event>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            fuel: Cell::new(256),
            events: Vec::new(),
        }
    }

    pub fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        self.events
            .push(Event::Open {
                kind: TreeKind::ErrorTree,
            });
        mark
    }

    pub fn close(&mut self, mark: MarkOpened, kind: TreeKind) -> MarkClosed {
        self.events[mark.index] = Event::Open { kind };
        self.events.push(Event::Close);
        MarkClosed {
            index: mark.index,
        }
    }

    pub fn open_before(&mut self, mark: MarkClosed) -> MarkOpened {
        let opened = MarkOpened {
            index: mark.index,
        };
        self.events.insert(
            mark.index,
            Event::Open {
                kind: TreeKind::ErrorTree,
            },
        );
        opened
    }

    pub fn advance(&mut self) {
        assert!(!self.eof());
        self.fuel.set(256);
        self.events.push(Event::Advance);
        self.pos += 1;
    }

    pub fn eof(&self) -> bool {
        self.at(TokenKind::Eof)
    }

    pub fn nth(&self, lookahead: usize) -> TokenKind {
        if self.fuel.get() == 0 {
            panic!("parser is stuck");
        }
        self.fuel.set(self.fuel.get() - 1);
        self.tokens
            .get(self.pos + lookahead)
            .map_or(TokenKind::Eof, |token| token.kind)
    }

    pub fn at(&self, kind: TokenKind) -> bool {
        self.nth(0) == kind
    }

    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn expect(&mut self, kind: TokenKind) {
        if self.eat(kind) {
            return;
        }
        let mark = self.open();
        self.close(mark, TreeKind::ErrorTree);
    }

    #[allow(dead_code)]
    pub fn advance_with_error(&mut self) {
        let mark = self.open();
        if !self.eof() {
            self.advance();
        }
        self.close(mark, TreeKind::ErrorTree);
    }

    pub fn build_tree(self) -> Tree {
        let mut tokens = self.tokens.into_iter();
        let mut events = self.events;
        let mut stack = Vec::new();

        assert!(matches!(events.pop(), Some(Event::Close)));

        for event in events {
            match event {
                Event::Open { kind } => {
                    stack.push(Tree {
                        kind,
                        children: Vec::new(),
                    });
                }
                Event::Close => {
                    let tree = stack.pop().expect("unbalanced close event");
                    stack
                        .last_mut()
                        .expect("unbalanced close event")
                        .children
                        .push(Child::Tree(tree));
                }
                Event::Advance => {
                    let token = tokens.next().expect("missing token for advance event");
                    stack
                        .last_mut()
                        .expect("advance outside of tree")
                        .children
                        .push(Child::Token(token));
                }
            }
        }

        assert_eq!(stack.len(), 1);
        assert!(tokens.next().is_none());

        stack.pop().expect("missing root tree")
    }
}
