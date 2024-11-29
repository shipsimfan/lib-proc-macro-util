mod r#const;
mod lifetime;
mod r#type;

mod parse;
mod to_static;
mod to_tokens;

pub use lifetime::LifetimeParam;
pub use r#const::{ConstParam, ConstParamValue};
pub use r#type::TypeParam;

/// The kind a generic parameter is
#[derive(Debug, Clone)]
pub enum GenericParamKind<'a> {
    /// The generic parameter is a constant
    Const(ConstParam<'a>),

    /// The generic parameter is a lifetime
    Lifetime(LifetimeParam<'a>),

    /// The generic parameter is a type
    Type(TypeParam<'a>),
}
