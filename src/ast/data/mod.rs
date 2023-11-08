mod r#enum;
mod field;
mod fields;
mod fields_named;
mod fields_unnamed;
mod r#struct;
mod union;
mod variant;

pub use field::Field;
pub use fields::Fields;
pub use fields_named::FieldsNamed;
pub use fields_unnamed::FieldsUnnamed;
pub use r#enum::DataEnum;
pub use r#struct::DataStruct;
pub use union::DataUnion;
pub use variant::Variant;

#[derive(Clone)]
pub enum Data {
    Struct(DataStruct),
    Enum(DataEnum),
    Union(DataUnion),
}
