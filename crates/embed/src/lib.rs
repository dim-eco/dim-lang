#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn ping() -> String {
    format!("dim-lang-runtime {}", dim_lang_runtime::version())
}
