use crate::ast::Lifetime;

impl<'a> std::fmt::Display for Lifetime<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lifetime::Identifier(quote, identifier) => {
                quote.fmt(f)?;
                identifier.fmt(f)
            }
            Lifetime::IdentifierOwned(quote, identifier) => {
                quote.fmt(f)?;
                identifier.fmt(f)
            }
            Lifetime::Underscore(quote, underscore) => {
                quote.fmt(f)?;
                underscore.fmt(f)
            }
        }
    }
}
