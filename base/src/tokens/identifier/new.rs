use crate::{tokens::Identifier, Span};

impl Identifier {
    /// Creates a new [`Identifier`] from `ident`
    pub const fn new_raw(ident: proc_macro::Ident) -> Self {
        Identifier(ident)
    }

    /// Creates a new [`Identifier`] with `span`
    pub fn new_at(identifier: &str, span: Span) -> Self {
        Identifier::new_raw(proc_macro::Ident::new(identifier, span))
    }

    /// Creates a new [`Identifier`] with [`Span::call_site`]
    pub fn new(identifier: &str) -> Self {
        Identifier::new_at(identifier, Span::call_site())
    }
}
