use crate::{ast::WhereClauseItem, Generator, ToTokens};

impl<'a> ToTokens for WhereClauseItem<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            WhereClauseItem::Lifetime {
                lifetime,
                colon,
                bounds,
            } => {
                lifetime.to_tokens(generator);
                colon.to_tokens(generator);
                bounds.to_tokens(generator);
            }
            WhereClauseItem::Type {
                for_lifetimes,
                r#type,
                colon,
                bounds,
            } => {
                for_lifetimes.to_tokens(generator);
                r#type.to_tokens(generator);
                colon.to_tokens(generator);
                bounds.to_tokens(generator);
            }
        }
    }
}
