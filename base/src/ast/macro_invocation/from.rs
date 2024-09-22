use crate::{
    ast::{DelimTokenTree, MacroInvocation, SimplePath},
    tokens::Group,
    Token,
};

impl<'a> From<(SimplePath<'a>, Token![!], DelimTokenTree<'a>)> for MacroInvocation<'a> {
    fn from(value: (SimplePath<'a>, Token![!], DelimTokenTree<'a>)) -> Self {
        MacroInvocation {
            path: value.0,
            exclamation: value.1,
            group: value.2,
        }
    }
}

impl<'a> From<(SimplePath<'a>, Token![!], &'a Group)> for MacroInvocation<'a> {
    fn from(value: (SimplePath<'a>, Token![!], &'a Group)) -> Self {
        MacroInvocation {
            path: value.0,
            exclamation: value.1,
            group: value.2.into(),
        }
    }
}

impl<'a> From<(SimplePath<'a>, Token![!], Group)> for MacroInvocation<'a> {
    fn from(value: (SimplePath<'a>, Token![!], Group)) -> Self {
        MacroInvocation {
            path: value.0,
            exclamation: value.1,
            group: value.2.into(),
        }
    }
}

impl<'a> From<(SimplePath<'a>, DelimTokenTree<'a>)> for MacroInvocation<'a> {
    fn from(value: (SimplePath<'a>, DelimTokenTree<'a>)) -> Self {
        MacroInvocation {
            path: value.0,
            exclamation: Token![!](),
            group: value.1,
        }
    }
}

impl<'a> From<(SimplePath<'a>, &'a Group)> for MacroInvocation<'a> {
    fn from(value: (SimplePath<'a>, &'a Group)) -> Self {
        MacroInvocation {
            path: value.0,
            exclamation: Token![!](),
            group: value.1.into(),
        }
    }
}

impl<'a> From<(SimplePath<'a>, Group)> for MacroInvocation<'a> {
    fn from(value: (SimplePath<'a>, Group)) -> Self {
        MacroInvocation {
            path: value.0,
            exclamation: Token![!](),
            group: value.1.into(),
        }
    }
}
