use crate::ast::{GenericArg, GenericParamKind};

impl<'a> GenericParamKind<'a> {
    /// Convert this generic parameter kind into a generic argument
    pub fn to_generic_arg(&self) -> GenericArg<'a> {
        match self {
            GenericParamKind::Const(r#const) => GenericArg::Const(r#const.to_generic_arg()),
            GenericParamKind::Lifetime(lifetime) => GenericArg::Lifetime(lifetime.lifetime.clone()),
            GenericParamKind::Type(r#type) => GenericArg::Type(Box::new(r#type.to_type())),
        }
    }
}
