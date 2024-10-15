use crate::ast::items::Module;

mod parse;
mod to_tokens;

/// A type of item prefixed with a visibility
#[derive(Debug, Clone)]
pub enum VisItemKind<'a> {
    /// A container for other items
    Module(Module<'a>),
}
