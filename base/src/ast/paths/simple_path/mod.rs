use crate::Token;

mod segment;

mod display;
mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

pub use segment::SimplePathSegment;

/// Simple paths are used in visibility markers, attributes, macros, and `use` items.
#[derive(Debug, Clone)]
pub struct SimplePath<'a> {
    /// The optional leading `::` indicating this path is absolute
    pub leading: Option<Token![::]>,

    /// The first segment of the simple path
    pub first: SimplePathSegment<'a>,

    /// The remaining segments and separators of the path
    pub remaining: Vec<(Token![::], SimplePathSegment<'a>)>,
}

impl<'a> SimplePath<'a> {
    /// Pushes a new `segment` onto the end of this path
    pub fn push<T: Into<SimplePathSegment<'a>>>(&mut self, segment: T) {
        self.remaining.push((Token![::](), segment.into()));
    }
}
