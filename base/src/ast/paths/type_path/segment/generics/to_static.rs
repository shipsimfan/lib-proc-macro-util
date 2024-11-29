use crate::ast::TypePathSegmentGenerics;

impl<'a> TypePathSegmentGenerics<'a> {
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
