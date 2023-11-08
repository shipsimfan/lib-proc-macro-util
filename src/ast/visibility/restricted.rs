use crate::{
    ast::Path,
    tokens::{In, Parenthesis, Pub},
};

#[derive(Clone)]
pub struct VisibilityRestricted {
    pub r#pub: Pub,
    pub parentheses: Parenthesis,
    pub r#in: Option<In>,
    pub path: Box<Path>,
}
