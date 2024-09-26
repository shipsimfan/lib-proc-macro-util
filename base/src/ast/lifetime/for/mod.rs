use crate::{ast::GenericParams, Token};

pub struct ForLifetimes {
    r#for: Token![for],
    generics: GenericParams,
}
