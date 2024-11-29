use crate::{ast::SimplePath, Token};

mod display;
mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// A scope restricting where the item can be accessed
#[derive(Debug, Clone)]
pub enum VisibilityScope<'a> {
    /// The item can only be accessed from the same crate
    Crate(Token![crate]),

    /// The item can only be accessed from the same module
    _Self(Token![self]),

    /// The item can only be accessed from the super module
    Super(Token![super]),

    /// The item can only be accesed from the specified module
    Path(Token![in], SimplePath<'a>),
}
