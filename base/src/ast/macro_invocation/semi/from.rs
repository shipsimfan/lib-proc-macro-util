use crate::{
    ast::{MacroInvocationSemi, SimplePath},
    tokens::Group,
    Delimiter, Token,
};

impl<'a> From<(SimplePath<'a>, Token![!], &'a Group, Token![;])> for MacroInvocationSemi<'a> {
    fn from(value: (SimplePath<'a>, Token![!], &'a Group, Token![;])) -> Self {
        assert!(
            value.2.delimiter == Delimiter::Parenthesis || value.2.delimiter == Delimiter::Bracket
        );
        MacroInvocationSemi::ParenthesesOrBracket(value.0, value.1, value.2.into(), value.3)
    }
}

impl<'a> From<(SimplePath<'a>, Token![!], Group, Token![;])> for MacroInvocationSemi<'a> {
    fn from(value: (SimplePath<'a>, Token![!], Group, Token![;])) -> Self {
        assert!(
            value.2.delimiter == Delimiter::Parenthesis || value.2.delimiter == Delimiter::Bracket
        );
        MacroInvocationSemi::ParenthesesOrBracket(value.0, value.1, value.2.into(), value.3)
    }
}

impl<'a> From<(SimplePath<'a>, Token![!], &'a Group)> for MacroInvocationSemi<'a> {
    fn from(value: (SimplePath<'a>, Token![!], &'a Group)) -> Self {
        assert_eq!(value.2.delimiter, Delimiter::Brace);
        MacroInvocationSemi::Brace(value.0, value.1, value.2.into())
    }
}

impl<'a> From<(SimplePath<'a>, Token![!], Group)> for MacroInvocationSemi<'a> {
    fn from(value: (SimplePath<'a>, Token![!], Group)) -> Self {
        assert_eq!(value.2.delimiter, Delimiter::Brace);
        MacroInvocationSemi::Brace(value.0, value.1, value.2.into())
    }
}
