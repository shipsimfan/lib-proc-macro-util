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

pub use proc_macro::{Delimiter, Span};
pub use proc_macro_util_base::*;
pub use proc_macro_util_macros::*;
