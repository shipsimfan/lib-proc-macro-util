//! # Proc-macro util base
//! Base definitions for lib-proc-macro-util

#![deny(missing_docs)]
#![feature(proc_macro_span)]

extern crate proc_macro;

mod generating;
mod macros;
mod parsing;

pub mod ast;
pub mod tokens;

pub use generating::{generate, Generator, ToTokens};
pub use parsing::{parse, Error, ErrorMessage, Parse, Parser, Result, TokenBuffer};
