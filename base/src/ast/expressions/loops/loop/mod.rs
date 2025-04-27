mod kind;
mod label;

mod parse;
mod to_static;
mod to_tokens;

pub use kind::LoopExpressionKind;
pub use label::LoopLabel;

/// A type of expression that loops
#[derive(Debug, Clone)]
pub struct LoopExpression<'a> {
    /// A label naming the loop
    pub label: Option<LoopLabel<'a>>,

    /// The kind of loop this is
    pub kind: LoopExpressionKind<'a>,
}
