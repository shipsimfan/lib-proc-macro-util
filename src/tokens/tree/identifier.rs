use proc_macro::Span;

use crate::ToTokens;

/// An identifier
#[derive(Clone)]
pub struct Identifier(proc_macro::Ident);

impl Identifier {
    /// Creates a new [`Identifier`]
    ///
    /// ## Parameters
    ///  * `identifier` - The identifier
    ///  * `span` - The [`Span`] for the new identifier
    ///
    /// ## Return Value
    /// Returns the newly created [`Identifier`]
    pub fn new(identifier: &str, span: Span) -> Self {
        Identifier(proc_macro::Ident::new(identifier, span))
    }
}

impl From<proc_macro::Ident> for Identifier {
    fn from(ident: proc_macro::Ident) -> Self {
        Identifier(ident)
    }
}

impl Into<proc_macro::Ident> for Identifier {
    fn into(self) -> proc_macro::Ident {
        self.0
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, generator: &mut crate::Generator) {
        generator.identifier_string(&self.0.to_string(), self.0.span())
    }
}
