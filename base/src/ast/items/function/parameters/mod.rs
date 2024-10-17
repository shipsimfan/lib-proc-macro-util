use crate::Token;

mod param;
mod self_param;

pub use param::FunctionParam;
pub use self_param::SelfParam;

#[derive(Debug, Clone)]
pub enum FunctionParameters<'a> {
    OnlySelf {
        _self: SelfParam<'a>,
        comma: Option<Token![,]>,
    },
    Normal {
        _self: Option<(SelfParam<'a>, Token![,])>,
        first: FunctionParam<'a>,
        remaining: Vec<(Token![,], FunctionParam<'a>)>,
        ending: Option<Token![,]>,
    },
}
