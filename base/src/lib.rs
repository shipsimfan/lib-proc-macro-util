//! # Proc-macro util base
//! Base definitions for lib-proc-macro-util

#![deny(missing_docs)]
#![feature(proc_macro_span)]

extern crate proc_macro;

mod generating;
mod macros;
mod parsing;

//pub mod ast;
pub mod tokens;

pub use generating::{generate, into_token_stream, Generator, ToTokens};
pub use parsing::{collect_token_stream, parse, Error, ErrorMessage, Parse, Parser, Result};
pub use proc_macro::{Delimiter, Span};
