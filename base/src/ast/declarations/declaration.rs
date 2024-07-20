use crate::ast::Struct;

/// A declaration defining something
#[derive(Clone)]
pub enum Declaration<'a> {
    /// A definition of a [`Struct`]
    Struct(Struct<'a>),
}
