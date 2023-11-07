extern crate proc_macro;

mod error;

pub mod keyword;

pub mod parsing;

pub use error::{Error, Result};
