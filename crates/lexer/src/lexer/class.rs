pub(crate) fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

pub(crate) fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

pub(crate) fn is_unicode_punct(ch: char) -> bool {
    matches!(
        ch,
        '≡' | '⊕' | '⊙' | '⊗' | '∞' | '≤' | '≥' | '≠' | '±' | '×' | '÷'
    ) || ch.is_ascii_punctuation()
}
