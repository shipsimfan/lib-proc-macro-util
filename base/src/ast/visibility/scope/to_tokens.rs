use crate::{ast::VisibilityScope, Generator, ToTokens};

impl<'a> ToTokens for VisibilityScope<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            VisibilityScope::Crate(krate) => krate.to_tokens(generator),
            VisibilityScope::_Self(_self) => _self.to_tokens(generator),
            VisibilityScope::Super(_super) => _super.to_tokens(generator),
            VisibilityScope::Path(r#in, path) => {
                r#in.to_tokens(generator);
                path.to_tokens(generator);
            }
        }
    }
}
