use crate::{ast::items::CrateRef, Generator, ToTokens};

impl<'a> ToTokens for CrateRef<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            CrateRef::Identifier(identifier) => identifier.clone().to_tokens(generator),
            CrateRef::_Self(_self) => _self.to_tokens(generator),
        }
    }
}
