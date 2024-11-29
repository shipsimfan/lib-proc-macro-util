use crate::ast::TypeNoBounds;

impl<'a> TypeNoBounds<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TypeNoBounds<'static> {
        match self {
            TypeNoBounds::Parenthesized(parenthesized) => {
                TypeNoBounds::Parenthesized(parenthesized.into_static())
            }
            TypeNoBounds::ImplTraitOneBound(impl_trait) => {
                TypeNoBounds::ImplTraitOneBound(impl_trait.into_static())
            }
            TypeNoBounds::TraitObjectOneBound(trait_object) => {
                TypeNoBounds::TraitObjectOneBound(trait_object.into_static())
            }
            TypeNoBounds::Path(path) => TypeNoBounds::Path(path.into_static()),
            TypeNoBounds::Tuple(tuple) => TypeNoBounds::Tuple(tuple.into_static()),
            TypeNoBounds::Never(never) => TypeNoBounds::Never(never),
            TypeNoBounds::RawPointer(raw_pointer) => {
                TypeNoBounds::RawPointer(raw_pointer.into_static())
            }
            TypeNoBounds::Reference(reference) => TypeNoBounds::Reference(reference.into_static()),
            TypeNoBounds::Array(array) => TypeNoBounds::Array(array.into_static()),
            TypeNoBounds::Slice(slice) => TypeNoBounds::Slice(slice.into_static()),
            TypeNoBounds::Inferred(inferred) => TypeNoBounds::Inferred(inferred),
            TypeNoBounds::QualifiedPath(qualified_type) => {
                TypeNoBounds::QualifiedPath(qualified_type.into_static())
            }
            TypeNoBounds::MacroInvocation(macro_invocation) => {
                TypeNoBounds::MacroInvocation(macro_invocation.into_static())
            }
            TypeNoBounds::BareFunction(bare_function) => {
                TypeNoBounds::BareFunction(bare_function.into_static())
            }
        }
    }
}
