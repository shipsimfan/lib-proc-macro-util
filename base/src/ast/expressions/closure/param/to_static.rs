use crate::ast::{expressions::ClosureParam, OuterAttribute};

impl<'a> ClosureParam<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> ClosureParam<'static> {
        ClosureParam {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            pattern: self.pattern.into_static(),
            r#type: self
                .r#type
                .map(|(colon, r#type)| (colon, Box::new(r#type.into_static()))),
        }
    }
}
