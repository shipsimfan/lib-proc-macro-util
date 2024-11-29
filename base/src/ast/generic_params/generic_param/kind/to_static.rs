use crate::ast::GenericParamKind;

impl<'a> GenericParamKind<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> GenericParamKind<'static> {
        match self {
            GenericParamKind::Const(r#const) => GenericParamKind::Const(r#const.into_static()),
            GenericParamKind::Lifetime(lifetime) => {
                GenericParamKind::Lifetime(lifetime.into_static())
            }
            GenericParamKind::Type(r#type) => GenericParamKind::Type(r#type.into_static()),
        }
    }
}
