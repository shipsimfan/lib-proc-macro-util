use crate::{ast::FieldsNamed, tokens::Union};

#[derive(Clone)]
pub struct DataUnion {
    pub union: Union,
    pub fields: FieldsNamed,
}
