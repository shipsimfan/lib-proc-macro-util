use crate::{
    ast::{AssociatedConstant, AssociatedType, Constraint, Expression, Lifetime, Type},
    parsing::Parse,
    tokens::{Brace, Literal},
    Token,
};

#[derive(Clone)]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Constant(Expression),
    AssociatedType(AssociatedType),
    AssociatedConstant(AssociatedConstant),
    Constraint(Constraint),
}

impl<'a> Parse<'a> for GenericArgument {
    fn parse(parser: &mut crate::parsing::Parser<'a>) -> crate::Result<Self> {
        if parser.peek::<Lifetime>() && !parser.peek2::<Token![+]>() {
            return Ok(GenericArgument::Lifetime(parser.parse()?));
        }

        if parser.peek::<Literal>() || parser.peek::<Brace>() {
            return Expression::constant(parser).map(GenericArgument::Constant);
        }

        todo!();
        /*        let mut argument: Type = parser.parse()?;
        match argument {
            _ => {}
        }

        Ok(GenericArgument::Type(argument))*/
    }
}
