use crate::{ast::PatternWithoutRange, Generator, ToTokens};

impl<'a> ToTokens for PatternWithoutRange<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            PatternWithoutRange::Literal(literal) => literal.to_tokens(generator),
            PatternWithoutRange::Identifier(identifier) => identifier.to_tokens(generator),
            PatternWithoutRange::Wildcard(wildcard) => wildcard.to_tokens(generator),
            PatternWithoutRange::Rest(rest) => rest.to_tokens(generator),
            PatternWithoutRange::Reference(reference) => reference.to_tokens(generator),
            PatternWithoutRange::TupleStruct(tuple_struct) => tuple_struct.to_tokens(generator),
            PatternWithoutRange::Tuple(tuple) => tuple.to_tokens(generator),
            PatternWithoutRange::Grouped(grouped) => grouped.to_tokens(generator),
            PatternWithoutRange::Slice(slice) => slice.to_tokens(generator),
            PatternWithoutRange::Path(path) => path.to_tokens(generator),
            PatternWithoutRange::MacroInvocation(macro_invocation) => {
                macro_invocation.to_tokens(generator)
            }
        }
    }
}
