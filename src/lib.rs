//! # lib-proc-macro-util
//! A library to ease writing procedural macros
//!
//! ## Concepts
//! **TODO**
//!
//! ## Usage
//! **TODO**

#![deny(missing_docs)]
#![feature(proc_macro_span)]

extern crate proc_macro;

mod generating;
mod macros;
mod parsing;

pub mod tokens;

pub use generating::{generate, Generator, ToTokens};
pub use parsing::{parse, Error, ErrorMessage, Parse, Parser, Result};

pub(crate) use parsing::TokenBuffer;
