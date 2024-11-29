use crate::ast::MacroInvocationSemi;
use std::borrow::Cow;

impl<'a> MacroInvocationSemi<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> MacroInvocationSemi<'static> {
        match self {
            MacroInvocationSemi::ParenthesesOrBracket(path, exclamation, group, semi) => {
                MacroInvocationSemi::ParenthesesOrBracket(
                    path.into_static(),
                    exclamation,
                    Cow::Owned(match group {
                        Cow::Borrowed(borrowed) => borrowed.clone(),
                        Cow::Owned(owned) => owned,
                    }),
                    semi,
                )
            }
            MacroInvocationSemi::Brace(path, exclamation, group) => MacroInvocationSemi::Brace(
                path.into_static(),
                exclamation,
                Cow::Owned(match group {
                    Cow::Borrowed(borrowed) => borrowed.clone(),
                    Cow::Owned(owned) => owned,
                }),
            ),
        }
    }
}
