pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn lexer_version() -> &'static str {
    dim_lang_lexer::version()
}
