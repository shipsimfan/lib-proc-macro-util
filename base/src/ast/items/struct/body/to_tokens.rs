use crate::{ast::items::StructBody, Generator, ToTokens};

impl<'a> ToTokens for StructBody<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            StructBody::Normal {
                where_clause,
                fields,
            } => {
                where_clause.to_tokens(generator);
                fields.to_tokens(&mut generator.group_brace());
            }
            StructBody::Tuple {
                fields,
                where_clause,
                semi,
            } => {
                fields.to_tokens(&mut generator.group_parenthesis());
                where_clause.to_tokens(generator);
                semi.to_tokens(generator);
            }
            StructBody::Empty { where_clause, semi } => {
                where_clause.to_tokens(generator);
                semi.to_tokens(generator);
            }
        }
    }
}
