use crate::{
    tokens::{Group, Identifier},
    Delimiter, ErrorMessage, Generator, ToTokens, Token,
};

impl ToTokens for ErrorMessage {
    fn to_tokens(&self, generator: &mut Generator) {
        let (start, end) = (self.span.start(), self.span.end());

        <Token![::]>::new([start; 2]).to_tokens(generator);
        Identifier::new_at("core", start).to_tokens(generator);
        <Token![::]>::new([start; 2]).to_tokens(generator);
        Identifier::new_at("compile_error", start).to_tokens(generator);
        <Token![!]>::new([start]).to_tokens(generator);

        let mut group = Group::new_at(Delimiter::Parenthesis, end);
        let mut group_gen = Generator::new(&mut group.tokens);
        Identifier::new_at(&self.message, end).to_tokens(&mut group_gen);

        <Token![;]>::new([end]).to_tokens(generator);
    }
}
