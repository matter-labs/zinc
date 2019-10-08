//!
//! The transpiler converter.
//!

use parser::TypeVariant;

pub struct Converter {}

impl Converter {
    pub fn r#type(input: TypeVariant) -> String {
        match input {
            TypeVariant::Unit => "()".to_owned(),
            TypeVariant::Boolean => "Boolean".to_owned(),
            TypeVariant::IntegerSigned { .. } => "AllocatedNum<Bn256>".to_owned(),
            TypeVariant::IntegerUnsigned { .. } => "AllocatedNum<Bn256>".to_owned(),
            TypeVariant::Field => "AllocatedNum<Bn256>".to_owned(),
            TypeVariant::Array { type_variant, size } => {
                format!("[{}; {}]", Self::r#type(*type_variant), size)
            }
            TypeVariant::Tuple { type_variants } => format!(
                "({})",
                type_variants
                    .into_iter()
                    .map(Self::r#type)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            TypeVariant::Structure { identifier, .. } => identifier,
            TypeVariant::Alias { identifier } => identifier,
        }
    }
}
