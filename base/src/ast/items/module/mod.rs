use crate::{tokens::Identifier, Token};

mod body;

mod parse;
mod to_tokens;

pub use body::ModuleBody;

/// A container for items
#[derive(Debug, Clone)]
pub struct Module<'a> {
    /// If the module is unsafe
    pub r#unsafe: Option<Token![unsafe]>,

    /// The marker for this module
    pub r#mod: Token![mod],

    /// The name of the module
    pub identifier: &'a Identifier,

    /// The body of the module
    pub body: ModuleBody<'a>,
}
