use crate::ast::{Attribute, Block, Visibility};

mod argument;
mod pattern_type;
mod receiver;
mod signature;
mod variadic;

pub use argument::FunctionArgument;
pub use pattern_type::PatternType;
pub use receiver::Receiver;
pub use signature::Signature;
pub use variadic::Variadic;

#[derive(Clone)]
pub struct ItemFn {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub signature: Signature,
    pub block: Box<Block>,
}
