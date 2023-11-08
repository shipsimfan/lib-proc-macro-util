use crate::{
    ast::{Punctuated, WherePredicate},
    tokens::{Comma, Where},
};

#[derive(Clone)]
pub struct WhereClause {
    pub r#where: Where,
    pub predicate: Punctuated<WherePredicate, Comma>,
}
