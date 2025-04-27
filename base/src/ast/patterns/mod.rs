//! Definitions for all patterns in Rust

mod grouped;
mod identifier;
mod literal;
mod path;
mod range;
mod reference;
mod rest;
mod slice;
mod r#struct;
mod tuple;
mod tuple_struct;
mod wildcard;

mod no_top_alt;
mod pattern;
mod without_range;

pub use grouped::GroupedPattern;
pub use identifier::IdentifierPattern;
pub use literal::LiteralPattern;
pub use path::PathPattern;
pub use r#struct::*;
pub use range::*;
pub use reference::ReferencePattern;
pub use rest::RestPattern;
pub use slice::{SlicePattern, SlicePatternItems};
pub use tuple::{TuplePattern, TuplePatternItems};
pub use tuple_struct::{TupleStructItems, TupleStructPattern};
pub use wildcard::WildcardPattern;

pub use no_top_alt::PatternNoTopAlt;
pub use pattern::Pattern;
pub use without_range::PatternWithoutRange;
