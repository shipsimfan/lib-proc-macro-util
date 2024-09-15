use crate::{tokens::Group, Delimiter};

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (open, close) = match self.delimiter {
            Delimiter::Brace => ("{", "}"),
            Delimiter::Bracket => ("[", "]"),
            Delimiter::Parenthesis => ("(", ")"),
            Delimiter::None => ("", ""),
        };

        f.write_str(open)?;
        for token in &self.tokens {
            f.write_str(" ")?;
            token.fmt(f)?;
        }
        if self.tokens.len() > 0 {
            f.write_str(" ")?;
        }
        f.write_str(close)
    }
}
