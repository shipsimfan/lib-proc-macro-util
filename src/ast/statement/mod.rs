use crate::{ast::Expression, tokens::SemiColon};

mod item;
mod local;
mod r#macro;

pub use item::Item;
pub use local::{Local, LocalInit};
pub use r#macro::StatementMacro;

#[derive(Clone)]
pub enum Statement {
    Local(Local),
    Item(Item),
    Expression(Expression, Option<SemiColon>),
    Macro(StatementMacro),
}
