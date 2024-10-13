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
            TypeNoBounds::Array(array) => array.to_tokens(generator),
            TypeNoBounds::Slice(slice) => slice.to_tokens(generator),
            TypeNoBounds::Inferred(inferred) => inferred.to_tokens(generator),
            TypeNoBounds::QualifiedPath(qualified_type) => qualified_type.to_tokens(generator),
            TypeNoBounds::MacroInvocation(macro_invocation) => {
                macro_invocation.to_tokens(generator)
            }
            TypeNoBounds::BareFunction(bare_function) => bare_function.to_tokens(generator),
        }
    }
}
