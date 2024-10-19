use crate::{
    ast::{Lifetime, OuterAttribute, Type},
    Token,
};

mod parse;
mod to_tokens;

/// A function parameter defining the type of `self`
#[derive(Debug, Clone)]
pub struct SelfParam<'a> {
    /// Attributes affecting this parameter
    pub attributes: Vec<OuterAttribute<'a>>,

    /// Tokens indicating if this is a reference to `self`
    pub reference: Option<(Token![&], Option<Lifetime<'a>>)>,

    /// Token indicating if `self` is mutable
    pub r#mut: Option<Token![mut]>,

    /// The `self` token itself
    pub _self: Token![self],

    /// The type of `self`
    pub r#type: Option<(Token![:], Type<'a>)>,
}
