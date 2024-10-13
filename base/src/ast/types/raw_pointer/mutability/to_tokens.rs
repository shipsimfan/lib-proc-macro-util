use crate::{ast::types::RawPointerTypeMutability, Generator, ToTokens};

impl ToTokens for RawPointerTypeMutability {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            RawPointerTypeMutability::Mut(r#mut) => r#mut.to_tokens(generator),
            RawPointerTypeMutability::Const(r#const) => r#const.to_tokens(generator),
        }
    }
}
