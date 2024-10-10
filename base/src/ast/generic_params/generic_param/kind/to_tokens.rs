use crate::{ast::GenericParamKind, Generator, ToTokens};

impl<'a> ToTokens for GenericParamKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            GenericParamKind::Const(r#const) => r#const.to_tokens(generator),
            GenericParamKind::Lifetime(lifetime) => lifetime.to_tokens(generator),
            GenericParamKind::Type(r#type) => r#type.to_tokens(generator),
        }
    }
}
