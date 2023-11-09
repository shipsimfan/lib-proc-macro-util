use crate::{ast::Lifetime, tokens::Colon};

#[derive(Clone)]
pub struct Label {
    pub name: Lifetime,
    pub colon: Colon,
}
