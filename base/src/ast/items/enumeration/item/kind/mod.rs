use crate::ast::items::{StructFields, TupleFields};

mod parse;
mod to_static;
mod to_tokens;

/// The types associated with an enum variant
#[derive(Debug, Clone)]
pub enum EnumItemKind<'a> {
    /// The types are defined like a tuple
    Tuple(Option<TupleFields<'a>>),

    /// The types are defined like a struct
    Struct(Option<StructFields<'a>>),
}
