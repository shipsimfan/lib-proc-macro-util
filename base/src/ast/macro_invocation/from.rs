use crate::{
    ast::{MacroInvocation, SimplePath},
    tokens::Group,
    Token,
};
use std::borrow::Cow;

impl<'a> From<(SimplePath<'a>, Token![!], Cow<'a, Group>)> for MacroInvocation<'a> {
    fn from(value: (SimplePath<'a>, Token![!], Cow<'a, Group>)) -> Self {
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
            group: Cow::Borrowed(value.2),
        }
    }
}

impl<'a> From<(SimplePath<'a>, Token![!], Group)> for MacroInvocation<'a> {
    fn from(value: (SimplePath<'a>, Token![!], Group)) -> Self {
        MacroInvocation {
            path: value.0,
            exclamation: value.1,
            group: Cow::Owned(value.2),
        }
    }
}

impl<'a> From<(SimplePath<'a>, Cow<'a, Group>)> for MacroInvocation<'a> {
    fn from(value: (SimplePath<'a>, Cow<'a, Group>)) -> Self {
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
            group: Cow::Borrowed(value.1),
        }
    }
}

impl<'a> From<(SimplePath<'a>, Group)> for MacroInvocation<'a> {
    fn from(value: (SimplePath<'a>, Group)) -> Self {
        MacroInvocation {
            path: value.0,
            exclamation: Token![!](),
            group: Cow::Owned(value.1),
        }
    }
}
