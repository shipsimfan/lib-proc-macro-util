//! # Proc-macro util base
//! Base definitions for lib-proc-macro-util

#![deny(missing_docs)]
#![feature(proc_macro_span)]

extern crate proc_macro;

mod collect_token_stream;
mod generating;
mod macros;
mod parsing;

//pub mod ast;
pub mod tokens;

pub use collect_token_stream::collect_token_stream;
pub use generating::{generate, Generator, ToTokens};
pub use parsing::{parse, Error, ErrorMessage, Parse, Parser, Result};
pub use proc_macro::{Delimiter, Span};
