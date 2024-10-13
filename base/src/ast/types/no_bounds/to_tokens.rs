use crate::{ast::TypeNoBounds, Generator, ToTokens};

impl<'a> ToTokens for TypeNoBounds<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            TypeNoBounds::Parenthesized(parenthesized) => parenthesized.to_tokens(generator),
            TypeNoBounds::ImplTraitOneBound(impl_trait) => impl_trait.to_tokens(generator),
            TypeNoBounds::TraitObjectOneBound(trait_object) => trait_object.to_tokens(generator),
            TypeNoBounds::Path(path) => path.to_tokens(generator),
            TypeNoBounds::Tuple(tuple) => tuple.to_tokens(generator),
            TypeNoBounds::Never(never) => never.to_tokens(generator),
            TypeNoBounds::RawPointer(raw_pointer) => raw_pointer.to_tokens(generator),
            TypeNoBounds::Reference(reference) => reference.to_tokens(generator),
        }
    }
}
