use crate::ast::items::{Enumeration, ExternCrate, Function, Module, Struct, UseDeclaration};

mod parse;
mod to_static;
mod to_tokens;

/// A type of item prefixed with a visibility
#[derive(Debug, Clone)]
pub enum VisItemKind<'a> {
    /// A container for other items
    Module(Module<'a>),

    /// A reference to an external crate
    ExternCrate(ExternCrate<'a>),

    /// An import of one or more items
    Use(UseDeclaration<'a>),

    /// A function definition
    Function(Function<'a>),

    /// A structure definition
    Struct(Struct<'a>),

    /// An enum definition
    Enumeration(Enumeration<'a>),
}
