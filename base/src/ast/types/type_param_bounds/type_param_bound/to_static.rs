use crate::ast::TypeParamBound;

impl<'a> TypeParamBound<'a> {
    pub fn into_static(self) -> TypeParamBound<'static> {
        match self {
            TypeParamBound::Lifetime(lifetime) => TypeParamBound::Lifetime(lifetime.into_static()),
            TypeParamBound::Trait(trait_bound) => {
                TypeParamBound::Trait(Box::new(trait_bound.into_static()))
            }
        }
    }
}
