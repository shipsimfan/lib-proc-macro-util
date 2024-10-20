use crate::ast::items::{ExternCrate, Function, Module, UseDeclaration};

mod parse;
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
}
