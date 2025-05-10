use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A discriminant defining the value of a variant
#[derive(Debug, Clone)]
pub struct EnumItemDiscriminant<'a> {
    /// The equals sign introducing this discriminant
    pub equals: Token![=],

    /// The expression that this is equal to
    pub expression: Expression<'a>,
}
