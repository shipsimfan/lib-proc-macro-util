use crate::{ast::patterns::StructPatternFieldName, Generator, ToTokens};

impl<'a> ToTokens for StructPatternFieldName<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            StructPatternFieldName::Index(index, colon, pattern) => {
                index.to_tokens(generator);
                colon.to_tokens(generator);
                pattern.to_tokens(generator);
            }
            StructPatternFieldName::IdentifierPattern(identifier, colon, pattern) => {
                identifier.to_tokens(generator);
                colon.to_tokens(generator);
                pattern.to_tokens(generator);
            }
            StructPatternFieldName::Identifier(r#ref, r#mut, identifier) => {
                r#ref.to_tokens(generator);
                r#mut.to_tokens(generator);
                identifier.to_tokens(generator);
            }
        }
    }
}
