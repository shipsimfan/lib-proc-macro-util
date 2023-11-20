//! Representations of parts of Rust's AST to assist parsing and generating [`TokenStream`]s

// rustdoc imports
#[allow(unused_imports)]
use proc_macro::TokenStream;

mod expressions;
mod path;
mod punctuated;

pub use expressions::*;
pub use path::*;
pub use punctuated::Punctuated;
