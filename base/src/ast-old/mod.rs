//! Representations of parts of Rust's AST to assist parsing and generating [`TokenStream`]s

// rustdoc imports
#[allow(unused_imports)]
use proc_macro::TokenStream;

mod expressions;
mod generics;
mod items;
mod lifetime;
mod meta;
mod path;
mod punctuated;
mod r#type;
mod variable_name;
mod visibility;

pub use expressions::*;
pub use generics::*;
pub use items::*;
pub use lifetime::Lifetime;
pub use meta::Meta;
pub use path::*;
pub use punctuated::*;
pub use r#type::Type;
pub use variable_name::VariableName;
pub use visibility::Visibility;
