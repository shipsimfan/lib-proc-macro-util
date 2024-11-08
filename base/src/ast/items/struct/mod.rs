use crate::{ast::GenericParams, tokens::Identifier, Token};
use std::borrow::Cow;

mod body;
mod struct_fields;
mod tuple_fields;

pub use body::StructBody;
pub use struct_fields::{StructField, StructFields};
pub use tuple_fields::{TupleField, TupleFields};

/// An ordered set of child types
#[derive(Debug, Clone)]
pub struct Struct<'a> {
    /// The `struct` token indicating this is a struct
    pub r#struct: Token![struct],

    /// The name of the structure
    pub name: Cow<'a, Identifier>,

    /// Generic parameters affecting the structure
    pub generic_params: Option<GenericParams<'a>>,

    /// The body of the structure
    pub body: StructBody<'a>,
}
