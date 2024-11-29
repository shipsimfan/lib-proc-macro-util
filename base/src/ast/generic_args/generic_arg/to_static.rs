use crate::ast::GenericArg;

impl<'a> GenericArg<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> GenericArg<'static> {
        match self {
            GenericArg::Lifetime(lifetime) => GenericArg::Lifetime(lifetime.into_static()),
            GenericArg::Type(r#type) => GenericArg::Type(Box::new(r#type.into_static())),
            GenericArg::Const(r#const) => GenericArg::Const(r#const.into_static()),
            GenericArg::Binding(binding) => GenericArg::Binding(binding.into_static()),
            GenericArg::Bounds(bounds) => GenericArg::Bounds(bounds.into_static()),
        }
    }
}
