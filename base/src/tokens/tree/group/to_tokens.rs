use crate::{tokens::Group, ToTokens};

impl<'a> ToTokens for Group {
    fn to_tokens(self, generator: &mut crate::Generator) {
        generator.push(self.into())
    }
}
