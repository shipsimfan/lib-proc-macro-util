use crate::ast::OuterAttribute;

mod kind;

mod parse;
mod to_tokens;

pub use kind::*;

/// A generic parameter
pub struct GenericParam<'a> {
    /// Attributes modifying this parameter
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The parameter itself
    pub kind: GenericParamKind<'a>,
}
