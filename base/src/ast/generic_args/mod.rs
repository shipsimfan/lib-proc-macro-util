use crate::Token;

mod binding;
mod bounds;
mod r#const;
mod generic_arg;

mod parse;
mod to_static;
mod to_tokens;

pub use binding::GenericArgsBinding;
pub use bounds::GenericArgsBounds;
pub use generic_arg::GenericArg;
pub use r#const::GenericArgsConst;

/// A set of generic arguments
#[derive(Debug, Clone)]
pub struct GenericArgs<'a> {
    /// The marker for the start of generic arguments
    pub open: Token![<],

    /// The set of arguments and their separators
    pub args: Vec<(GenericArg<'a>, Token![,])>,

    /// The final required argument
    pub last_arg: GenericArg<'a>,

    /// The final separator
    pub last_comma: Option<Token![,]>,

    /// The marker for the end of generic arguments
    pub end: Token![>],
}
