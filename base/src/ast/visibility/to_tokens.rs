use crate::{ast::Visibility, Generator, ToTokens};

impl<'a> ToTokens for Visibility<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#pub.to_tokens(generator);
        if let Some(scope) = self.scope {
            scope.to_tokens(&mut generator.group_parenthesis());
        }
    }
}
