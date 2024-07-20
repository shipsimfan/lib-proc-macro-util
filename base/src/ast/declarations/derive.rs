use crate::ast::Struct;

/// A declaration which can have a trait derived for it
#[derive(Clone)]
pub enum Derive<'a> {
    /// A [`Struct`]
    Struct(Struct<'a>),
}
