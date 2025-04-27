use crate::ast::{patterns::StructPatternEtCetera, OuterAttribute};

impl<'a> StructPatternEtCetera<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructPatternEtCetera<'static> {
        StructPatternEtCetera {
            attributes: self
                .attributes
                .into_iter()
                .map(OuterAttribute::into_static)
                .collect(),
            dots: self.dots,
        }
    }
}
