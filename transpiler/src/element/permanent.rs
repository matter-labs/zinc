//!
//! Transpiler permanent element.
//!

use std::fmt;

use parser::TypeVariant;

use crate::element::Descriptor;

#[derive(Debug)]
pub struct Element {
    pub identifier: String,
    pub type_variant: TypeVariant,
    pub is_mutable: bool,
    pub descriptors: Vec<Descriptor>,
}

impl Element {
    pub fn new(identifier: String, type_variant: TypeVariant, is_mutable: bool) -> Self {
        Self {
            identifier,
            type_variant,
            is_mutable,
            descriptors: Default::default(),
        }
    }

    pub fn push_descriptor(&mut self, descriptor: Descriptor) {
        self.descriptors.push(descriptor);
    }

    pub fn type_variant(&self) -> TypeVariant {
        let mut type_variant = &self.type_variant;
        for descriptor in self.descriptors.iter() {
            type_variant = match type_variant {
                TypeVariant::Array { type_variant, .. } => &*type_variant,
                TypeVariant::Tuple { type_variants } => {
                    let field = match descriptor {
                        Descriptor::Tuple(field) => *field,
                        _ => panic!("1"),
                    };

                    &type_variants[field]
                }
                TypeVariant::Structure { fields, .. } => {
                    let field = match descriptor {
                        Descriptor::Structure(identifier) => identifier,
                        _ => panic!("1"),
                    };

                    fields.get(field).unwrap()
                }
                _ => panic!("1"),
            };
        }
        type_variant.to_owned()
    }
}

impl Into<String> for Element {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            "(".repeat(self.descriptors.len()),
            self.identifier,
            self.descriptors
                .iter()
                .map(|descriptor| descriptor.to_string() + ")")
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}
