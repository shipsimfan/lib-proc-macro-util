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

pub use base::*;
pub use macros::*;
pub use proc_macro::{Delimiter, Span};
