use crate::{ast::items::Enumeration, Generator, ToTokens};

impl<'a> ToTokens for Enumeration<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#enum.to_tokens(generator);
        self.name.to_tokens(generator);
        self.generic_params.to_tokens(generator);
        self.where_clause.to_tokens(generator);
        self.enum_items.to_tokens(&mut generator.group_brace());
    }
}
