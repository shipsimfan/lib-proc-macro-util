use crate::{ast::expressions::CompoundAssignmentOperator, Generator, ToTokens};

impl ToTokens for CompoundAssignmentOperator {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            CompoundAssignmentOperator::Add(add) => add.to_tokens(generator),
            CompoundAssignmentOperator::Sub(sub) => sub.to_tokens(generator),
            CompoundAssignmentOperator::Mul(mul) => mul.to_tokens(generator),
            CompoundAssignmentOperator::Div(div) => div.to_tokens(generator),
            CompoundAssignmentOperator::Mod(r#mod) => r#mod.to_tokens(generator),
            CompoundAssignmentOperator::And(and) => and.to_tokens(generator),
            CompoundAssignmentOperator::Or(or) => or.to_tokens(generator),
            CompoundAssignmentOperator::Xor(xor) => xor.to_tokens(generator),
            CompoundAssignmentOperator::Shl(shl) => shl.to_tokens(generator),
            CompoundAssignmentOperator::Shr(shr) => shr.to_tokens(generator),
        }
    }
}
