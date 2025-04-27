use crate::ast::{patterns::StructPatternField, OuterAttribute};

impl<'a> StructPatternField<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructPatternField<'static> {
        StructPatternField {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            name: self.name.into_static(),
        }
    }
}
