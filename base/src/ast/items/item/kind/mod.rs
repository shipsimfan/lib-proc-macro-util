use crate::ast::{MacroItem, VisItem};

mod parse;
mod to_tokens;

/// Kinds that items can be
#[derive(Debug, Clone)]
pub enum ItemKind<'a> {
    /// The item may be prefixed with a visibility
    Vis(VisItem<'a>),

    /// The item involves a macro
    Macro(MacroItem),
}
