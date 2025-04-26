use crate::Token;

mod param;
mod self_param;

mod parse;
mod to_static;
mod to_tokens;

pub use param::FunctionParam;
pub use self_param::SelfParam;

/// A set of function parameters
#[derive(Debug, Clone)]
pub enum FunctionParameters<'a> {
    /// The only parameter is `self`
    OnlySelf {
        /// The self parameter
        _self: SelfParam<'a>,

        /// An optional trailing comma
        comma: Option<Token![,]>,
    },

    /// The parameters are normal
    Normal {
        /// An initial self parameter
        _self: Option<(SelfParam<'a>, Token![,])>,

        /// The first parameter
        first: FunctionParam<'a>,

        /// The rest of the parameters and their separators
        remaining: Vec<(Token![,], FunctionParam<'a>)>,

        /// A final optional comma
        ending: Option<Token![,]>,
    },
}
