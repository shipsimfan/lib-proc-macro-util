use crate::ast::MacroInvocationSemi;

mod parse;
mod to_static;
mod to_tokens;

/// An item which involves a macro
#[derive(Debug, Clone)]
pub enum MacroItem<'a> {
    /// Invocation of a macro
    MacroInvocationSemi(MacroInvocationSemi<'a>),
}
