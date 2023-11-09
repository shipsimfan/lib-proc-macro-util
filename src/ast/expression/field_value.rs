use crate::{
    ast::{Attribute, Expression, Member},
    tokens::Colon,
};

#[derive(Clone)]
pub struct FieldValue {
    pub attributes: Vec<Attribute>,
    pub member: Member,
    pub colon: Option<Colon>,
    pub expression: Expression,
}
