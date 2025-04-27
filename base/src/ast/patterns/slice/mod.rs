mod items;

mod parse;
mod to_static;
mod to_tokens;

pub use items::SlicePatternItems;

/// Slice patterns can match both arrays of fixed size and slices of dynamic size.
#[derive(Debug, Clone)]
pub struct SlicePattern<'a> {
    /// The items the be matched in the slice
    pub items: Option<SlicePatternItems<'a>>,
}
