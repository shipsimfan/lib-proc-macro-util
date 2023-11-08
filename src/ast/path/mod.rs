use crate::ast::Punctuated;

mod arguments;
mod segment;
mod seperator;

pub use arguments::PathArguments;
pub use segment::PathSegment;
pub use seperator::PathSeperator;

#[derive(Clone)]
pub struct Path {
    pub leading_colon: Option<PathSeperator>,
    pub segments: Punctuated<PathSegment, PathSeperator>,
}
