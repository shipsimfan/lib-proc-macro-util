//! # Proc-macro util base
//! Base definitions for lib-proc-macro-util

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

extern crate proc_macro;

mod generating;
mod macros;
mod parsing;

pub mod ast;
pub mod tokens;

pub use generating::{generate, into_token_stream, Generator, ToTokens};
pub use parsing::{collect_token_stream, parse, Error, ErrorMessage, Parse, Parser, Result};
pub use proc_macro::{Delimiter, Spacing, Span};
