use crate::{ast::items::ModuleBody, Generator, ToTokens};

impl<'a> ToTokens for ModuleBody<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ModuleBody::None(semi) => semi.to_tokens(generator),
            ModuleBody::Some { attributes, items } => {
                let mut generator = generator.group_brace();
                attributes.to_tokens(&mut generator);
                items.to_tokens(&mut generator);
            }
        }
    }
}
