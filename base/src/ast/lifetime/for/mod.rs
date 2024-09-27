use crate::{ast::GenericParams, Token};

pub struct ForLifetimes<'a> {
    r#for: Token![for],
    generics: GenericParams<'a>,
}
