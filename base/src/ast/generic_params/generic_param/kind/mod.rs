mod r#const;
mod lifetime;
mod r#type;

pub use lifetime::LifetimeParam;
pub use r#const::ConstParam;
pub use r#type::TypeParam;

pub enum GenericParamKind<'a> {
    Const(ConstParam<'a>),
    Lifetime(LifetimeParam<'a>),
    Type(TypeParam<'a>),
}
