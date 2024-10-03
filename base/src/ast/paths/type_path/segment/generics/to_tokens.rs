use crate::{ast::TypePathSegmentGenerics, Generator, ToTokens};

impl<'a> ToTokens for TypePathSegmentGenerics<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            TypePathSegmentGenerics::GenericArgs(generic_args) => todo!(),
            TypePathSegmentGenerics::TypePathFn(type_path_fn) => type_path_fn.to_tokens(generator),
        }
    }
}
