use crate::tokens::{Ident, Span};

#[derive(Clone)]
pub struct Lifetime {
    pub apostrophe: Span,
    pub ident: Ident,
}
