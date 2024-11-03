use crate::ast::{OuterAttribute, Type, Visibility};

mod parse;
mod to_tokens;

/// A single field in a tuple-style struct
#[derive(Debug, Clone)]
pub struct TupleField<'a> {
    /// The attributes affecting this field
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The visibility of this field
    pub visibility: Option<Visibility<'a>>,

    /// The type of this field
    pub r#type: Type<'a>,
}
