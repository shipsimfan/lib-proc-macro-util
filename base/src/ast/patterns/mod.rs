//! Definitions for all patterns in Rust

mod literal;
mod range;

mod no_top_alt;
mod without_range;

pub use literal::LiteralPattern;
pub use range::*;

pub use no_top_alt::PatternNoTopAlt;
pub use without_range::PatternWithoutRange;
