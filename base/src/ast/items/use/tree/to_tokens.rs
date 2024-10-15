use crate::{ast::items::UseTree, Generator, ToTokens};

impl<'a> ToTokens for UseTree<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            UseTree::All { prefix, asterick } => {
                prefix.to_tokens(generator);
                asterick.to_tokens(generator);
            }
            UseTree::SubTree { prefix, trees } => {
                prefix.to_tokens(generator);
                trees.to_tokens(&mut generator.group_brace());
            }
            UseTree::Leaf { path, r#as } => {
                path.to_tokens(generator);
                r#as.to_tokens(generator);
            }
        }
    }
}
