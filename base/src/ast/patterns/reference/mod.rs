use crate::{
    ast::{expressions::BorrowAmpersand, PatternWithoutRange},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// Reference patterns dereference the pointers that are being matched and, thus, borrow them.
#[derive(Debug, Clone)]
pub struct ReferencePattern<'a> {
    /// The ampersand indicating this is a reference
    pub ampersand: BorrowAmpersand,

    /// Is this reference mutable?
    pub r#mut: Option<Token![mut]>,

    /// The pattern to match for dereferencing
    pub pattern: Box<PatternWithoutRange<'a>>,
}
