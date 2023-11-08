use crate::{ast::PathArguments, parsing::Parse, tokens::Ident, Token};

#[derive(Clone)]
pub struct PathSegment {
    pub ident: Ident,
    pub arguments: PathArguments,
}

impl<T: Into<Ident>> From<T> for PathSegment {
    fn from(ident: T) -> Self {
        PathSegment {
            ident: ident.into(),
            arguments: PathArguments::None,
        }
    }
}

impl<'a> Parse<'a> for PathSegment {
    fn parse(parser: &mut crate::parsing::Parser<'a>) -> crate::Result<Self> {
        if parser.peek::<Token![super]>()
            || parser.peek::<Token![self]>()
            || parser.peek::<Token![crate]>()
            || parser.peek::<Token![try]>()
        {
            let ident: Ident = parser.parse()?;
            return Ok(PathSegment::from(ident));
        }

        let ident = parser.parse()?;

        Ok(
            if parser.peek::<Token![<]>() && !parser.peek::<Token![<=]>()
                || parser.peek::<Token![::]>() && parser.peek3::<Token![<]>()
            {
                PathSegment {
                    ident,
                    arguments: PathArguments::AngleBrackets(parser.parse()?),
                }
            } else {
                PathSegment::from(ident)
            },
        )
    }
}
