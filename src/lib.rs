//! # lib-proc-macro-util
//! A library to ease writing procedural macros
//!
//! ## Concepts
//! **TODO**
//!
//! ## Usage
//! **TODO**

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

pub use proc_macro_util_base::{
    ast, collect_token_stream, generate, into_token_stream, parse, proc_macro_derive,
    proc_macro_function, tokens, Delimiter, Error, ErrorMessage, Generator, Parse, Parser, Result,
    Spacing, Span, ToTokens, Token,
};
pub use proc_macro_util_macros::*;
