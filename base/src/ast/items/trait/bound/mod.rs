use crate::{
    ast::{ForLifetimes, TypePath},
    Token,
};

pub struct TraitBound<'a> {
    pub delimited: bool,
    pub question: Option<Token![?]>,
    pub for_lifetimes: Option<ForLifetimes<'a>>,
    pub path: TypePath<'a>,
}
