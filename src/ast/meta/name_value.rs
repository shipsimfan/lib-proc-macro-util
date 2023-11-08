use crate::{
    ast::{Expression, Path},
    tokens::Equals,
};

#[derive(Clone)]
pub struct MetaNameValue {
    pub path: Path,
    pub equals: Equals,
    pub value: Expression,
}
