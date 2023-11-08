use crate::ast::{FieldsNamed, FieldsUnnamed};

#[derive(Clone)]
pub enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}
