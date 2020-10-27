//!
//! The variable binding.
//!

#[cfg(test)]
mod tests;

pub mod error;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_syntax::BindingPattern;
use zinc_syntax::BindingPatternVariant;
use zinc_syntax::Identifier;

use crate::semantic::analyzer::statement::r#fn::Context as FnAnalyzerContext;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::memory_type::MemoryType;
use crate::semantic::scope::Scope;

use self::error::Error;

///
/// The binding object namespace.
///
pub struct Binder {}

///
/// The single variable binding.
///
#[derive(Debug, Clone)]
pub struct Binding {
    /// The bound variable name.
    pub identifier: Identifier,
    /// Whether the bound variable is mutable.
    pub is_mutable: bool,
    /// Whether the binding is a wildcard.
    pub is_wildcard: bool,
    /// The bound variable r#type.
    pub r#type: Type,
}

impl Binding {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(identifier: Identifier, is_mutable: bool, is_wildcard: bool, r#type: Type) -> Self {
        Self {
            identifier,
            is_mutable,
            is_wildcard,
            r#type,
        }
    }
}

impl Binder {
    ///
    /// Validates a binding pattern and returns the variable declaration list.
    ///
    pub fn bind_variables(
        pattern: BindingPattern,
        r#type: Type,
        memory_type: MemoryType,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Vec<Binding>, SemanticError> {
        match pattern.variant {
            BindingPatternVariant::Binding {
                identifier,
                is_mutable,
            } => {
                Scope::define_variable(
                    scope,
                    identifier.clone(),
                    is_mutable,
                    r#type.clone(),
                    memory_type,
                )?;

                Ok(vec![Binding::new(identifier, is_mutable, false, r#type)])
            }
            BindingPatternVariant::BindingList { bindings } => {
                let types = match r#type {
                    Type::Tuple(tuple) if tuple.types.len() == bindings.len() => tuple.types,
                    r#type => {
                        return Err(SemanticError::Binding(Error::ExpectedTuple {
                            location: pattern.location,
                            expected: bindings.len(),
                            found: r#type.to_string(),
                        }));
                    }
                };

                let mut result = Vec::with_capacity(bindings.len());
                for (pattern, r#type) in bindings.into_iter().zip(types.into_iter()) {
                    result.extend(Self::bind_variables(
                        pattern,
                        r#type,
                        memory_type,
                        scope.clone(),
                    )?);
                }
                Ok(result)
            }
            BindingPatternVariant::Wildcard => Ok(vec![Binding::new(
                Identifier::new(pattern.location, "_".to_owned()),
                false,
                true,
                r#type,
            )]),
        }
    }

    ///
    /// Validates a binding pattern and returns the variable declaration list.
    ///
    pub fn bind_arguments(
        bindings: Vec<zinc_syntax::Binding>,
        context: FnAnalyzerContext,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Vec<Binding>, SemanticError> {
        let mut result = Vec::with_capacity(bindings.len());

        for (index, binding) in bindings.into_iter().enumerate() {
            match binding.pattern.variant {
                BindingPatternVariant::Binding {
                    identifier,
                    is_mutable,
                } if identifier.is_self_lowercase() => {
                    if index != 0 {
                        return Err(SemanticError::Binding(Error::FunctionMethodSelfNotFirst {
                            location: identifier.location,
                            name: identifier.name,
                            position: index + 1,
                        }));
                    }

                    let r#type = match &*scope
                        .borrow()
                        .resolve_item(
                            &Identifier::new(
                                identifier.location,
                                Keyword::SelfUppercase.to_string(),
                            ),
                            true,
                        )?
                        .borrow()
                    {
                        ScopeItem::Type(r#type) => r#type.define()?,
                        item => {
                            return Err(SemanticError::Element(ElementError::Type(
                                TypeError::AliasDoesNotPointToType {
                                    location: identifier.location,
                                    found: item.to_string(),
                                },
                            )))
                        }
                    };

                    if !r#type.is_instantiatable(false) {
                        return Err(SemanticError::Element(ElementError::Type(
                            TypeError::InstantiationForbidden {
                                location: identifier.location,
                                found: r#type.to_string(),
                            },
                        )));
                    }

                    let memory_type = match context {
                        FnAnalyzerContext::Contract => MemoryType::ContractInstance,
                        FnAnalyzerContext::Module => MemoryType::Stack,
                        FnAnalyzerContext::Implementation => MemoryType::Stack,
                    };

                    Scope::define_variable(
                        scope.clone(),
                        identifier.clone(),
                        is_mutable,
                        r#type.clone(),
                        memory_type,
                    )?;

                    result.push(Binding::new(identifier, is_mutable, false, r#type));
                }
                BindingPatternVariant::Binding {
                    identifier,
                    is_mutable,
                } => {
                    let r#type = binding
                        .r#type
                        .ok_or(TypeError::TypeRequired {
                            location: identifier.location,
                            identifier: identifier.name.to_owned(),
                        })
                        .map_err(ElementError::Type)
                        .map_err(SemanticError::Element)?;
                    let r#type = Type::try_from_syntax(r#type, scope.clone())?;

                    if !r#type.is_instantiatable(false) {
                        return Err(SemanticError::Element(ElementError::Type(
                            TypeError::InstantiationForbidden {
                                location: identifier.location,
                                found: r#type.to_string(),
                            },
                        )));
                    }

                    let memory_type = match r#type {
                        Type::Contract(_) => MemoryType::ContractInstance,
                        _ => MemoryType::Stack,
                    };

                    Scope::define_variable(
                        scope.clone(),
                        identifier.clone(),
                        is_mutable,
                        r#type.clone(),
                        memory_type,
                    )?;

                    result.push(Binding::new(identifier, is_mutable, false, r#type));
                }
                BindingPatternVariant::BindingList { .. } => {
                    return Err(SemanticError::Binding(
                        Error::FunctionArgumentDestructuringUnavailable {
                            location: binding.location,
                        },
                    ))
                }
                BindingPatternVariant::Wildcard => {
                    let r#type = binding
                        .r#type
                        .ok_or(TypeError::TypeRequired {
                            location: binding.location,
                            identifier: "_".to_owned(),
                        })
                        .map_err(ElementError::Type)
                        .map_err(SemanticError::Element)?;
                    let r#type = Type::try_from_syntax(r#type, scope.clone())?;

                    if !r#type.is_instantiatable(false) {
                        return Err(SemanticError::Element(ElementError::Type(
                            TypeError::InstantiationForbidden {
                                location: binding.pattern.location,
                                found: r#type.to_string(),
                            },
                        )));
                    }

                    result.push(Binding::new(
                        Identifier::new(binding.pattern.location, "_".to_owned()),
                        false,
                        true,
                        r#type,
                    ));
                }
            }
        }

        Ok(result)
    }
}
