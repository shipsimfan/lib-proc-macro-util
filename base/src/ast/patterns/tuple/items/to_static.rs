use crate::ast::patterns::TuplePatternItems;

impl<'a> TuplePatternItems<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> TuplePatternItems<'static> {
        match self {
            TuplePatternItems::Single(pattern, comma) => {
                TuplePatternItems::Single(Box::new(pattern.into_static()), comma)
            }
            TuplePatternItems::Rest(rest) => TuplePatternItems::Rest(rest),
            TuplePatternItems::Multiple {
                first,
                first_comma,
                second,
                remaining,
                last,
            } => TuplePatternItems::Multiple {
                first: Box::new(first.into_static()),
                first_comma,
                second: Box::new(second.into_static()),
                remaining: remaining
                    .into_iter()
                    .map(|(comma, pattern)| (comma, pattern.into_static()))
                    .collect(),
                last,
            },
        }
    }
}
