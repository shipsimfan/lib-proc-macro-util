mod infinite;
mod iterator;
mod predicate;

mod r#loop;

pub use infinite::InfiniteLoopExpression;
pub use iterator::IteratorLoopExpression;
pub use predicate::PredicateLoopExpression;

pub use r#loop::{LoopExpression, LoopExpressionKind, LoopLabel};
