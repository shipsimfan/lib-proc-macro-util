use crate::ast::Type;

impl<'a> Type<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Type<'static> {
        match self {
            Type::Parenthesized(parenthesized) => Type::Parenthesized(parenthesized.into_static()),
            Type::ImplTrait(impl_trait) => Type::ImplTrait(impl_trait.into_static()),
            Type::ImplTraitOneBound(impl_trait_one_bound) => {
                Type::ImplTraitOneBound(impl_trait_one_bound.into_static())
            }
            Type::TraitObject(trait_object) => Type::TraitObject(trait_object.into_static()),
            Type::TraitObjectOneBound(trait_object_one_bound) => {
                Type::TraitObjectOneBound(trait_object_one_bound.into_static())
            }
            Type::Path(path) => Type::Path(path.into_static()),
            Type::Tuple(tuple) => Type::Tuple(tuple.into_static()),
            Type::Never(never) => Type::Never(never),
            Type::RawPointer(raw_pointer) => Type::RawPointer(raw_pointer.into_static()),
            Type::Reference(reference) => Type::Reference(reference.into_static()),
            Type::Array(array) => Type::Array(array.into_static()),
            Type::Slice(slice) => Type::Slice(slice.into_static()),
            Type::Inferred(inferred) => Type::Inferred(inferred),
            Type::QualifiedPath(qualified_path) => {
                Type::QualifiedPath(qualified_path.into_static())
            }
            Type::MacroInvocation(macro_invocation) => {
                Type::MacroInvocation(macro_invocation.into_static())
            }
            Type::BareFunction(bare_function) => Type::BareFunction(bare_function.into_static()),
        }
    }
}
