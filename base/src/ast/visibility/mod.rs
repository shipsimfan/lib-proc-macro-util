use crate::Token;

mod scope;

mod display;
mod from;
mod new;
mod parse;
mod to_tokens;

pub use scope::VisibilityScope;

/// Defines where an item can be accessed from
pub struct Visibility<'a> {
    /// The "pub" token identifying the visibility
    pub r#pub: Token![pub],

    /// A scope restricting where the item can be accessed
    pub scope: Option<VisibilityScope<'a>>,
}
