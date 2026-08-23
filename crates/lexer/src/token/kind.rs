#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
    // ---- trivia -------------------------------------------------------
    // Kept in the stream (not skipped) so that the formatter and the LSP
    // can round-trip the source. A filtering iterator drops them for the
    // parser.
    /// Spaces and tabs. Never contains a line break.
    Whitespace,
    /// One or more line breaks. Separate from `Whitespace` because some
    /// constructs may be newline-sensitive.
    Newline,
    /// `// ...` to end of line.
    LineComment,
    /// `/* ... */`. `terminated: false` means the file ended inside it.
    BlockComment { terminated: bool, level: u32 },

    // ---- atoms --------------------------------------------------------
    // These cannot be per-character: they consume a variable-length run
    // that only the lexer can delimit correctly.
    /// Identifier. Keywords are NOT separate variants — see note below.
    Ident,
    /// Integer literal. `base` covers `0x` / `0o` / `0b` / decimal.
    Int { base: NumBase, empty_digits: bool },
    /// Float literal, e.g. `222.333`. Note `1..2` must lex as Int, Dot,
    /// Dot, Int — not as a float followed by a dot.
    Decimal,
    /// `"..."` string.
    Str { terminated: bool },
    /// Backtick template string used for dim fragments and error messages.
    /// Interpolation holes (`$node_id`) are re-lexed in a second pass over
    /// the span, so the outer token stays a single atom.
    TemplateStr { terminated: bool },

    // ---- delimiters ---------------------------------------------------
    // Flat open/close tokens, no token tree. Balancing is the parser's job.
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    // ---- punctuation: exactly one character each -----------------------
    Plus,      // +
    Minus,     // -   (`->` = Minus{Joint} + Gt)
    Star,      // *
    Slash,     // /
    Percent,   // %
    Eq,        // =   (`==` = Eq{Joint} + Eq, `:=` = Colon{Joint} + Eq)
    Bang,      // !   (constraint marker, and `!=`)
    Lt,        // <   (`<=`, `<<`, and the generics sugar `Pair<A, B>`)
    Gt,        // >
    Amp,       // &   (trait bounds `A & B`, and `&&`)
    Pipe,      // |   (closure params `|a, b|`, and `||`)
    Caret,     // ^
    Tilde,     // ~
    Dot,       // .   (`.key` DSL member, `..` range, field access)
    Comma,     // ,
    Semi,      // ;
    Colon,     // :   (`::`, `:=`, type ascription, trailing-block call)
    At,        // @   (`@wire`, `@renamed_from`)
    Pound,     // #
    Dollar,    // $   (fragment placeholders)
    Question,  // ?
    Backslash, // \

    // ---- fallback ------------------------------------------------------
    /// Any other Unicode punctuation or symbol: `≡`, `⊕`, `⊙`, `⊗`, `∞`.
    /// Promote one to its own variant once it becomes real syntax.
    Punct(char),
    /// A character the lexer cannot classify at all. Never rejected here —
    /// the parser reports it, so the LSP still gets a full token stream.
    Unknown(char),

    Eof,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NumBase {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}
