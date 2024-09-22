use crate::ast::SimplePath;

mod input;

mod from;
mod new;
mod parse;
mod to_tokens;

pub use input::AttrInput;

/// An attribute is a general, free-form metadatum that is interpreted according to name,
/// convention, language, and compiler version
#[derive(Debug, Clone)]
pub struct Attr<'a> {
    /// The path identifiying the attribute
    pub path: SimplePath<'a>,

    /// The value of the attribute
    pub input: Option<AttrInput<'a>>,
}
