use crate::ast::TypePathSegmentGenerics;

impl<'a> TypePathSegmentGenerics<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TypePathSegmentGenerics<'static> {
        match self {
            TypePathSegmentGenerics::GenericArgs(generic_args) => {
                TypePathSegmentGenerics::GenericArgs(generic_args.into_static())
            }
            TypePathSegmentGenerics::TypePathFn(type_path_fn) => {
                TypePathSegmentGenerics::TypePathFn(type_path_fn.into_static())
            }
        }
    }
}
