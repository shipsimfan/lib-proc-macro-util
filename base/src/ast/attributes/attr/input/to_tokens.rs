use crate::{ast::AttrInput, Generator, ToTokens};

impl<'a> ToTokens for AttrInput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            AttrInput::Group(group) => group.clone().to_tokens(generator),
            AttrInput::OwnedGroup(group) => group.to_tokens(generator),
            AttrInput::Expression(equals, expression) => {
                equals.to_tokens(generator);
                expression.to_tokens(generator);
            }
        }
    }
}
