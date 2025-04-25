use crate::{ast::expressions::BorrowAmpersand, Generator, ToTokens};

impl ToTokens for BorrowAmpersand {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            BorrowAmpersand::Single(ampersand) => ampersand.to_tokens(generator),
            BorrowAmpersand::Double(double_ampersand) => double_ampersand.to_tokens(generator),
        }
    }
}
