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
            PatternWithoutRange::MacroInvocation(macro_invocation) => {
                PatternWithoutRange::MacroInvocation(macro_invocation.into_static())
            }
        }
    }
}
