pub type Pos = u64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Span {
    pub lo: Pos,
    pub hi: Pos,
}

/// Whether this token is glued to the one that follows it.
///
/// Only meaningful for punctuation. `:` in `:=` is `Joint`; `:` in `key : Id`
/// is `Alone`. Without this the parser cannot tell `&&` from `& &`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Spacing {
    Joint,
    Alone,
}
