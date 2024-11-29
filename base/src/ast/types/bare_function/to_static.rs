use crate::ast::{
    types::{BareFunctionReturnType, BareFunctionType, MaybeNamedFunctionParameters},
    ForLifetimes,
};

impl<'a> BareFunctionType<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> BareFunctionType<'static> {
        BareFunctionType {
            for_lifetimes: self.for_lifetimes.map(ForLifetimes::into_static),
            qualifiers: self.qualifiers.into_static(),
            r#fn: self.r#fn,
            parameters: self
                .parameters
                .map(MaybeNamedFunctionParameters::into_static),
            return_type: self.return_type.map(BareFunctionReturnType::into_static),
        }
    }
}
