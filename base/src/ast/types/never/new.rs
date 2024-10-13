use crate::{ast::types::NeverType, Token};

impl NeverType {
    /// Creates a new [`NeverType`] with [`Span::call_site`](crate::Span::call_site)
    pub fn new() -> Self {
        NeverType(Token![!]())
    }
}
