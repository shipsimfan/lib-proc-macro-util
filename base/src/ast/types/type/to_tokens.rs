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
        }
    }
}