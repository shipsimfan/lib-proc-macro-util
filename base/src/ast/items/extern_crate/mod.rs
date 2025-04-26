use crate::{ast::MaybeIdentifier, Token};

mod r#ref;

mod parse;
mod to_static;
mod to_tokens;

pub use r#ref::CrateRef;

/// A reference to an external crate
#[derive(Debug, Clone)]
pub struct ExternCrate<'a> {
    /// The external marker
    pub r#extern: Token![extern],

    /// The marker this is an external crate
    pub krate: Token![crate],

    /// The name of the external crate
    pub crate_ref: CrateRef<'a>,

    /// An optional rename of the crate
    pub as_clause: Option<(Token![as], MaybeIdentifier<'a>)>,
}
