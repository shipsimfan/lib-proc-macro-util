use crate::{ast::items::Function, Generator, ToTokens};

impl<'a> ToTokens for Function<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.qualifiers.to_tokens(generator);
        self.r#fn.to_tokens(generator);
        self.name.clone().to_tokens(generator);
        self.generic_params.to_tokens(generator);
        self.parameters
            .to_tokens(&mut generator.group_parenthesis());
        self.return_type.to_tokens(generator);
        self.where_clause.to_tokens(generator);
        self.body.to_tokens(generator);
    }
}
