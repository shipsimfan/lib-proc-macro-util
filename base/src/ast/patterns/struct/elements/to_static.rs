use crate::ast::patterns::{StructPatternElements, StructPatternEtCetera};

impl<'a> StructPatternElements<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> StructPatternElements<'static> {
        match self {
            StructPatternElements::Fields(fields, et_cetera) => StructPatternElements::Fields(
                fields.into_static(),
                et_cetera.map(|(comma, et_cetera)| {
                    (comma, et_cetera.map(StructPatternEtCetera::into_static))
                }),
            ),
            StructPatternElements::EtCetera(et_cetera) => {
                StructPatternElements::EtCetera(et_cetera.into_static())
            }
        }
    }
}
