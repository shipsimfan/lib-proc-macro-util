//! Definitions for all patterns in Rust

mod identifier;
mod literal;
mod range;
mod wildcard;

mod no_top_alt;
mod without_range;

pub use identifier::IdentifierPattern;
pub use literal::LiteralPattern;
pub use range::*;
pub use wildcard::WildcardPattern;

pub use no_top_alt::PatternNoTopAlt;
pub use without_range::PatternWithoutRange;
