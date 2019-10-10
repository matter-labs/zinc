//!
//! Transpiler output type.
//!

use parser::TypeVariant;

pub struct Output {
    pub r#type: String,
}

impl From<TypeVariant> for Output {
    fn from(type_variant: TypeVariant) -> Self {
        let r#type = match type_variant {
            TypeVariant::Unit => "()".to_owned(),
            TypeVariant::Boolean => "Boolean".to_owned(),
            TypeVariant::IntegerSigned { .. } => "AllocatedNum<Bn256>".to_owned(),
            TypeVariant::IntegerUnsigned { .. } => "AllocatedNum<Bn256>".to_owned(),
            TypeVariant::Field => "AllocatedNum<Bn256>".to_owned(),
            TypeVariant::Array { type_variant, size } => {
                let r#type: String = Self::from(*type_variant).into();
                format!("[{}; {}]", r#type, size)
            }
            TypeVariant::Tuple { type_variants } => format!(
                "({})",
                type_variants
                    .into_iter()
                    .map(|type_variant| Self::from(type_variant).into())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            TypeVariant::Structure { identifier, .. } => identifier,
            TypeVariant::Alias { identifier } => identifier,
        };

        Self { r#type }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        self.r#type
    }
}
