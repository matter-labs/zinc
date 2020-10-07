//!
//! The function call semantic analyzer.
//!

pub mod r#type;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::element::Element as GeneratorExpressionElement;
use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::lexical::token::location::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::intrinsic::error::Error as IntrinsicFunctionError;
use crate::semantic::element::r#type::function::intrinsic::Function as IntrinsicFunctionType;
use crate::semantic::element::r#type::function::test::error::Error as TestFunctionError;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;

use self::r#type::Type as CallType;

///
/// The function call semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes the function call.
    ///
    /// Returns the semantic element and the intermediate representation.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        operand_1: Element,
        operand_2: Element,
        call_type: CallType,
        location: Location,
    ) -> Result<(Element, GeneratorExpressionElement), Error> {
        let function_location = operand_1.location();

        let function = match operand_1 {
            Element::Type(Type::Function(function)) => function,
            Element::Path(path) => match *Scope::resolve_path(scope.clone(), &path)?.borrow() {
                ScopeItem::Type(ref r#type) => {
                    let r#type = r#type.define()?;

                    match r#type {
                        Type::Function(function) => function,
                        r#type => {
                            return Err(Error::Element(ElementError::Type(TypeError::Function(
                                FunctionError::NonCallable {
                                    location: function_location.unwrap_or(location),
                                    name: r#type.to_string(),
                                },
                            ))))
                        }
                    }
                }
                ref item => {
                    return Err(Error::Element(ElementError::Type(TypeError::Function(
                        FunctionError::NonCallable {
                            location: function_location.unwrap_or(location),
                            name: item.to_string(),
                        },
                    ))));
                }
            },
            operand => {
                return Err(Error::Element(ElementError::Type(TypeError::Function(
                    FunctionError::NonCallable {
                        location: function_location.unwrap_or(location),
                        name: operand.to_string(),
                    },
                ))));
            }
        };

        let mut argument_list = match operand_2 {
            Element::ArgumentList(values) => values,
            _ => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        };

        if let CallType::Method { ref instance, .. } = call_type {
            argument_list.arguments.insert(0, *instance.to_owned());
        }

        let mut input_size = 0;
        for element in argument_list.arguments.iter() {
            input_size += Type::from_element(element, scope.clone())?.size();
        }

        let (element, intermediate) = match function {
            FunctionType::Intrinsic(function) => {
                if function.requires_exclamation_mark() && !matches!(call_type, CallType::MacroLike)
                {
                    return Err(Error::Element(ElementError::Type(TypeError::Function(
                        FunctionError::Intrinsic(IntrinsicFunctionError::ExclamationMarkMissing {
                            location: function_location.unwrap_or(location),
                            function: function.identifier(),
                        }),
                    ))));
                }

                match function {
                    IntrinsicFunctionType::Debug(function) => {
                        let (return_type, format, argument_types) = function
                            .call(function_location, argument_list)
                            .map_err(|error| {
                                Error::Element(ElementError::Type(TypeError::Function(error)))
                            })?;

                        let element = Element::Value(
                            Value::try_from_type(&return_type, false, None)
                                .map_err(ElementError::Value)
                                .map_err(Error::Element)?,
                        );

                        let intermediate =
                            GeneratorExpressionOperator::call_debug(format, argument_types);

                        (
                            element,
                            GeneratorExpressionElement::Operator {
                                location: function_location
                                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                                operator: intermediate,
                            },
                        )
                    }
                    IntrinsicFunctionType::Require(function) => {
                        let (return_type, message) = function
                            .call(function_location, argument_list)
                            .map_err(|error| {
                                Error::Element(ElementError::Type(TypeError::Function(error)))
                            })?;

                        let element = Element::Value(
                            Value::try_from_type(&return_type, false, None)
                                .map_err(ElementError::Value)
                                .map_err(Error::Element)?,
                        );

                        let intermediate = GeneratorExpressionOperator::call_assert(message);

                        (
                            element,
                            GeneratorExpressionElement::Operator {
                                location: function_location
                                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                                operator: intermediate,
                            },
                        )
                    }
                    IntrinsicFunctionType::StandardLibrary(function) => {
                        if let CallType::MacroLike = call_type {
                            return Err(Error::Element(ElementError::Type(TypeError::Function(
                                FunctionError::Intrinsic(IntrinsicFunctionError::Unknown {
                                    location: function_location.unwrap_or(location),
                                    function: function.identifier().to_owned(),
                                }),
                            ))));
                        }

                        let intrinsic_identifier = function.library_identifier();

                        let return_type =
                            function
                                .call(function_location, argument_list)
                                .map_err(|error| {
                                    Error::Element(ElementError::Type(TypeError::Function(error)))
                                })?;

                        let element = Element::Value(
                            Value::try_from_type(&return_type, false, None)
                                .map_err(ElementError::Value)
                                .map_err(Error::Element)?,
                        );

                        let intermediate = GeneratorExpressionOperator::call_library(
                            intrinsic_identifier,
                            input_size,
                            return_type.size(),
                        );

                        (
                            element,
                            GeneratorExpressionElement::Operator {
                                location: function_location
                                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                                operator: intermediate,
                            },
                        )
                    }
                    IntrinsicFunctionType::ZkSyncLibrary(function) => {
                        if let CallType::MacroLike = call_type {
                            return Err(Error::Element(ElementError::Type(TypeError::Function(
                                FunctionError::Intrinsic(IntrinsicFunctionError::Unknown {
                                    location: function_location.unwrap_or(location),
                                    function: function.identifier().to_owned(),
                                }),
                            ))));
                        }

                        let intrinsic_identifier = function.library_identifier();

                        let return_type =
                            function
                                .call(function_location, argument_list)
                                .map_err(|error| {
                                    Error::Element(ElementError::Type(TypeError::Function(error)))
                                })?;

                        let element = Element::Value(
                            Value::try_from_type(&return_type, false, None)
                                .map_err(ElementError::Value)
                                .map_err(Error::Element)?,
                        );

                        let intermediate = GeneratorExpressionOperator::call_library(
                            intrinsic_identifier,
                            input_size,
                            return_type.size(),
                        );

                        (
                            element,
                            GeneratorExpressionElement::Operator {
                                location: function_location
                                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                                operator: intermediate,
                            },
                        )
                    }
                }
            }
            FunctionType::Runtime(function) => {
                if let CallType::MacroLike = call_type {
                    return Err(Error::Element(ElementError::Type(TypeError::Function(
                        FunctionError::Intrinsic(IntrinsicFunctionError::Unknown {
                            location,
                            function: function.identifier,
                        }),
                    ))));
                }

                if let CallType::Method {
                    instance: _,
                    is_mutable: is_instance_mutable,
                } = call_type
                {
                    let is_first_argument_mutable = function
                        .formal_params
                        .get(0)
                        .map(|instance| instance.1)
                        .unwrap_or_default();

                    if !is_instance_mutable && is_first_argument_mutable {
                        return Err(Error::Element(ElementError::Type(TypeError::Function(
                            FunctionError::CallingMutableFromImmutable {
                                location,
                                function: function.identifier,
                            },
                        ))));
                    }
                }

                let location = function.location;
                let type_id = function.type_id;

                let return_type = function.call(argument_list).map_err(|error| {
                    Error::Element(ElementError::Type(TypeError::Function(error)))
                })?;

                let element = Element::Value(
                    Value::try_from_type(&return_type, false, None)
                        .map_err(ElementError::Value)
                        .map_err(Error::Element)?,
                );

                let intermediate = GeneratorExpressionOperator::call(type_id, input_size);

                (
                    element,
                    GeneratorExpressionElement::Operator {
                        location,
                        operator: intermediate,
                    },
                )
            }
            FunctionType::Constant(function) => {
                if let CallType::MacroLike = call_type {
                    return Err(Error::Element(ElementError::Type(TypeError::Function(
                        FunctionError::Intrinsic(IntrinsicFunctionError::Unknown {
                            location,
                            function: function.identifier,
                        }),
                    ))));
                }

                let arguments = function.validate(argument_list).map_err(|error| {
                    Error::Element(ElementError::Type(TypeError::Function(error)))
                })?;

                let constant = function.call(arguments, scope)?;

                let intermediate = GeneratorConstant::try_from_semantic(&constant)
                    .map(GeneratorExpressionOperand::Constant)
                    .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);

                (
                    Element::Constant(constant),
                    GeneratorExpressionElement::Operand(intermediate),
                )
            }
            FunctionType::Test(function) => {
                return Err(Error::Element(ElementError::Type(TypeError::Function(
                    FunctionError::Test(TestFunctionError::CallForbidden {
                        location: function_location.unwrap_or(location),
                        function: function.identifier,
                    }),
                ))));
            }
        };

        Ok((element, intermediate))
    }
}
