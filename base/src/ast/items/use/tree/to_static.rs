use crate::ast::items::UseTree;

impl<'a> UseTree<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> UseTree<'static> {
        match self {
            UseTree::All { prefix, asterick } => UseTree::All {
                prefix: prefix
                    .map(|(prefix, sep)| (prefix.map(|prefix| prefix.into_static()), sep)),
                asterick,
            },
            UseTree::SubTree { prefix, trees } => UseTree::SubTree {
                prefix: prefix
                    .map(|(prefix, sep)| (prefix.map(|prefix| prefix.into_static()), sep)),
                trees: trees.map(|(first, remaining, last)| {
                    (
                        Box::new(first.into_static()),
                        remaining
                            .into_iter()
                            .map(|(comma, tree)| (comma, tree.into_static()))
                            .collect(),
                        last,
                    )
                }),
            },
            UseTree::Leaf { path, r#as } => UseTree::Leaf {
                path: path.into_static(),
                r#as: r#as.map(|(r#as, identifier)| (r#as, identifier.into_static())),
            },
        }
    }
}
