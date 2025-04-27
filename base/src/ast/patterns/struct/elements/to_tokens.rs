use crate::{ast::patterns::StructPatternElements, Generator, ToTokens};

impl<'a> ToTokens for StructPatternElements<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            StructPatternElements::Fields(fields, et_cetera) => {
                fields.to_tokens(generator);
                et_cetera.to_tokens(generator);
            }
            StructPatternElements::EtCetera(et_cetera) => et_cetera.to_tokens(generator),
        }
    }
}
