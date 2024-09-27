use crate::{ast::GenericArg, Generator, ToTokens};

impl<'a> ToTokens for GenericArg<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            GenericArg::Lifetime(lifetime) => lifetime.to_tokens(generator),
            GenericArg::Type(r#type) => r#type.to_tokens(generator),
            GenericArg::Const(r#const) => r#const.to_tokens(generator),
            GenericArg::Binding(binding) => binding.to_tokens(generator),
            GenericArg::Bounds(bounds) => bounds.to_tokens(generator),
        }
    }
}
