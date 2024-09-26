use crate::{ast::ForLifetimes, Token};

pub struct TraitBound {
    pub delimited: bool,
    pub question: Option<Token![?]>,
    pub for_lifetimes: Option<ForLifetimes>,
    pub path: TypePath,
}
