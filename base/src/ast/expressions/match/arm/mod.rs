use crate::ast::{OuterAttribute, Pattern};

mod guard;

mod parse;
mod to_static;
mod to_tokens;

pub use guard::MatchArmGuard;

/// A single arm of a match statement
#[derive(Debug, Clone)]
pub struct MatchArm<'a> {
    /// The attributes effecting this match arm
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The pattern to match
    pub pattern: Pattern<'a>,

    /// The guard restricting the values to match
    pub guard: Option<MatchArmGuard<'a>>,
}
