use crate::{ast::GenericParams, tokens::Identifier, Token};
use std::borrow::Cow;

mod body;
mod struct_fields;
mod tuple_fields;

pub use body::StructBody;
pub use struct_fields::{StructField, StructFields};
pub use tuple_fields::{TupleField, TupleFields};

#[derive(Debug, Clone)]
pub struct Struct<'a> {
    pub r#struct: Token![struct],
    pub name: Cow<'a, Identifier>,
    pub generic_params: Option<GenericParams<'a>>,
    pub body: StructBody<'a>,
}
