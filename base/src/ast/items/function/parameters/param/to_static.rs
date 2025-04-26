use crate::ast::{items::FunctionParam, OuterAttribute};

impl<'a> FunctionParam<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> FunctionParam<'static> {
        match self {
            FunctionParam::Pattern {
                attributes,
                pattern,
                colon,
                r#type,
            } => FunctionParam::Pattern {
                attributes: attributes
                    .into_iter()
                    .map(OuterAttribute::into_static)
                    .collect(),
                pattern: pattern.into_static(),
                colon,
                r#type: r#type.into_static(),
            },
            FunctionParam::Variadic { attributes, dots } => FunctionParam::Variadic {
                attributes: attributes
                    .into_iter()
                    .map(OuterAttribute::into_static)
                    .collect(),
                dots,
            },
        }
    }
}
