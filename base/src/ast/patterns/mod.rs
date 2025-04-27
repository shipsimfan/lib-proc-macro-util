//! Definitions for all patterns in Rust

mod identifier;
mod literal;
mod path;
mod range;
mod reference;
mod rest;
mod wildcard;

mod no_top_alt;
mod without_range;

pub use identifier::IdentifierPattern;
pub use literal::LiteralPattern;
pub use path::PathPattern;
pub use range::*;
pub use reference::ReferencePattern;
pub use rest::RestPattern;
pub use wildcard::WildcardPattern;

pub use no_top_alt::PatternNoTopAlt;
pub use without_range::PatternWithoutRange;
