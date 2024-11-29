use crate::ast::{types::MaybeNamedFunctionParameters, OuterAttribute};

impl<'a> MaybeNamedFunctionParameters<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> MaybeNamedFunctionParameters<'static> {
        MaybeNamedFunctionParameters {
            first: self.first.into_static(),
            remaining: self
                .remaining
                .into_iter()
                .map(|(separator, parameter)| (separator, parameter.into_static()))
                .collect(),
            variadic: self
                .variadic
                .into_iter()
                .map(|(separator, attributes, dots)| {
                    (
                        separator,
                        attributes
                            .into_iter()
                            .map(OuterAttribute::into_static)
                            .collect(),
                        dots,
                    )
                })
                .collect(),
            ending: self.ending,
        }
    }
}
