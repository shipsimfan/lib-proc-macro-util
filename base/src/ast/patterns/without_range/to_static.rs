use crate::ast::PatternWithoutRange;

impl<'a> PatternWithoutRange<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> PatternWithoutRange<'static> {
        match self {
            PatternWithoutRange::Literal(literal) => {
                PatternWithoutRange::Literal(literal.into_static())
            }
            PatternWithoutRange::Identifier(identifier) => {
                PatternWithoutRange::Identifier(identifier.into_static())
            }
            PatternWithoutRange::Wildcard(wildcard) => PatternWithoutRange::Wildcard(wildcard),
            PatternWithoutRange::Rest(rest) => PatternWithoutRange::Rest(rest),
            PatternWithoutRange::Reference(reference) => {
                PatternWithoutRange::Reference(reference.into_static())
            }
            PatternWithoutRange::TupleStruct(tuple_struct) => {
                PatternWithoutRange::TupleStruct(tuple_struct.into_static())
            }
            PatternWithoutRange::Tuple(tuple) => PatternWithoutRange::Tuple(tuple.into_static()),
            PatternWithoutRange::Grouped(grouped) => {
                PatternWithoutRange::Grouped(grouped.into_static())
            }
            PatternWithoutRange::Slice(slice) => PatternWithoutRange::Slice(slice.into_static()),
            PatternWithoutRange::Path(path) => PatternWithoutRange::Path(path.into_static()),
            PatternWithoutRange::MacroInvocation(macro_invocation) => {
                PatternWithoutRange::MacroInvocation(macro_invocation.into_static())
            }
        }
    }
}
