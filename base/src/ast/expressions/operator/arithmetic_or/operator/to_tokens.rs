use crate::{ast::expressions::ArithmeticOrLogicalOperator, Generator, ToTokens};

impl ToTokens for ArithmeticOrLogicalOperator {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ArithmeticOrLogicalOperator::Add(add) => add.to_tokens(generator),
            ArithmeticOrLogicalOperator::Sub(sub) => sub.to_tokens(generator),
            ArithmeticOrLogicalOperator::Mul(mul) => mul.to_tokens(generator),
            ArithmeticOrLogicalOperator::Div(div) => div.to_tokens(generator),
            ArithmeticOrLogicalOperator::Mod(r#mod) => r#mod.to_tokens(generator),
            ArithmeticOrLogicalOperator::And(and) => and.to_tokens(generator),
            ArithmeticOrLogicalOperator::Or(or) => or.to_tokens(generator),
            ArithmeticOrLogicalOperator::Xor(xor) => xor.to_tokens(generator),
            ArithmeticOrLogicalOperator::Shl(shl) => shl.to_tokens(generator),
            ArithmeticOrLogicalOperator::Shr(shr) => shr.to_tokens(generator),
        }
    }
}
