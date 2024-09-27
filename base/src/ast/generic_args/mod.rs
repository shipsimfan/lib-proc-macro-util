use crate::Token;

mod binding;
mod bounds;
mod r#const;
mod generic_arg;

pub use binding::GenericArgsBinding;
pub use bounds::GenericArgsBounds;
pub use generic_arg::GenericArg;
pub use r#const::GenericArgsConst;

pub struct GenericArgs<'a> {
    pub open: Token![<],
    pub args: Vec<(GenericArg<'a>, Token![,])>,
    pub last_arg: GenericArg<'a>,
    pub last_comma: Option<Token![,]>,
    pub end: Token![>],
}
