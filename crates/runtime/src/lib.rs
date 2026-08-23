pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn parser_version() -> &'static str {
    dim_lang_parser::version()
}
