//!
//! Transpiler output type element.
//!

use parser::TypeVariant;

pub struct Output {}

impl Output {
    pub fn output(type_variant: TypeVariant) -> String {
        match type_variant {
            TypeVariant::Unit => "()".to_owned(),
            TypeVariant::Boolean => "Boolean".to_owned(),
            TypeVariant::IntegerSigned { .. } => "AllocatedNum<Bn256>".to_owned(),
            TypeVariant::IntegerUnsigned { .. } => "AllocatedNum<Bn256>".to_owned(),
            TypeVariant::Field => "AllocatedNum<Bn256>".to_owned(),
            TypeVariant::Array { type_variant, size } => {
                format!("[{}; {}]", Self::output(*type_variant), size)
            }
            TypeVariant::Tuple { type_variants } => format!(
                "({})",
                type_variants
                    .into_iter()
                    .map(Self::output)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            TypeVariant::Structure { .. } => unimplemented!(),
            TypeVariant::Enumeration { .. } => unimplemented!(),
            TypeVariant::Function { .. } => unimplemented!(),
            TypeVariant::Alias { identifier } => identifier,
        }
    }
}
