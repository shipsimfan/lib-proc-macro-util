use crate::{ast::Type, Generator, ToTokens};

impl<'a> ToTokens for Type<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            Type::Parenthesized(parenthesized) => parenthesized.to_tokens(generator),
            Type::ImplTrait(impl_trait) => impl_trait.to_tokens(generator),
            Type::ImplTraitOneBound(impl_trait_one_bound) => {
                impl_trait_one_bound.to_tokens(generator)
            }
            Type::TraitObject(trait_object) => trait_object.to_tokens(generator),
            Type::TraitObjectOneBound(trait_object_one_bound) => {
                trait_object_one_bound.to_tokens(generator)
            }
            Type::Path(path) => path.to_tokens(generator),
            Type::Tuple(tuple) => tuple.to_tokens(generator),
            Type::Never(never) => never.to_tokens(generator),
            Type::RawPointer(raw_pointer) => raw_pointer.to_tokens(generator),
            Type::Reference(reference) => reference.to_tokens(generator),
            Type::Array(array) => array.to_tokens(generator),
        }
    }
}
