//!
//! The function call semantic analyzer.
//!

pub mod r#type;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Location;

use crate::generator::expression::element::Element as GeneratorExpressionElement;
use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::generator::expression::operand::Operand as GeneratorExpressionOperand;
use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::generator::r#type::contract_field::ContractField as GeneratorContractField;
use crate::semantic::element::r#type::function::intrinsic::Function as IntrinsicFunctionType;
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
                            return Err(Error::FunctionNonCallable {
                                location: function_location.unwrap_or(location),
                                name: r#type.to_string(),
                            })
                        }
                    }
                }
                ref item => {
                    return Err(Error::FunctionNonCallable {
                        location: function_location.unwrap_or(location),
                        name: item.to_string(),
                    });
                }
            },
            operand => {
                return Err(Error::FunctionNonCallable {
                    location: function_location.unwrap_or(location),
                    name: operand.to_string(),
                });
            }
        };

        let mut argument_list = match operand_2 {
            Element::ArgumentList(values) => values,
            _ => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
        };

        let is_called_with_exclamation_mark = matches!(call_type, CallType::MacroLike);

        if let CallType::Method {
            instance,
            is_mutable,
        } = call_type
        {
            argument_list.arguments.insert(0, *instance);

            if !is_mutable && function.is_mutable() {
                return Err(Error::FunctionCallMutableFromImmutable {
                    location,
                    function: function.identifier(),
                });
            }
        }

        let mut input_size = 0;
        for element in argument_list.arguments.iter() {
            input_size += Type::from_element(element, scope.clone())?.size();
        }

        let (element, intermediate) = match function {
            FunctionType::Intrinsic(function) => {
                if function.requires_exclamation_mark() && !is_called_with_exclamation_mark {
                    return Err(Error::FunctionExpectedExclamationMark {
                        location: function_location.unwrap_or(location),
                        function: function.identifier(),
                    });
                }

                match function {
                    IntrinsicFunctionType::Debug(function) => {
                        let (return_type, format, argument_types) =
                            function.call(function_location.unwrap_or(location), argument_list)?;

                        let element =
                            Value::try_from_type(&return_type, false, None).map(Element::Value)?;

                        let intermediate =
                            GeneratorExpressionOperator::call_debug(format, argument_types);

                        (
                            element,
                            GeneratorExpressionElement::Operator {
                                location: function_location.unwrap_or(location),
                                operator: intermediate,
                            },
                        )
                    }
                    IntrinsicFunctionType::Require(function) => {
                        let (return_type, message) =
                            function.call(function_location.unwrap_or(location), argument_list)?;

                        let element =
                            Value::try_from_type(&return_type, false, None).map(Element::Value)?;

                        let intermediate = GeneratorExpressionOperator::call_require(message);

                        (
                            element,
                            GeneratorExpressionElement::Operator {
                                location: function_location.unwrap_or(location),
                                operator: intermediate,
                            },
                        )
                    }
                    IntrinsicFunctionType::ContractFetch(function) => {
                        let return_type =
                            function.call(function_location.unwrap_or(location), argument_list)?;

                        let element =
                            Value::try_from_type(&return_type, false, None).map(Element::Value)?;

                        let contract_fields: Vec<GeneratorContractField> = match return_type {
                            Type::Contract(contract) => contract
                                .fields
                                .iter()
                                .filter_map(GeneratorContractField::try_from_semantic)
                                .collect(),
                            _type => panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
                        };

                        let intermediate =
                            GeneratorExpressionOperator::call_contract_fetch(contract_fields);

                        (
                            element,
                            GeneratorExpressionElement::Operator {
                                location: function_location.unwrap_or(location),
                                operator: intermediate,
                            },
                        )
                    }
                    IntrinsicFunctionType::ContractTransfer(function) => {
                        let intrinsic_identifier = function.library_identifier;

                        let return_type =
                            function.call(function_location.unwrap_or(location), argument_list)?;

                        let element =
                            Value::try_from_type(&return_type, false, None).map(Element::Value)?;

                        let intermediate = GeneratorExpressionOperator::call_library(
                            intrinsic_identifier,
                            input_size,
                            return_type.size(),
                        );

                        (
                            element,
                            GeneratorExpressionElement::Operator {
                                location: function_location.unwrap_or(location),
                                operator: intermediate,
                            },
                        )
                    }
                    IntrinsicFunctionType::StandardLibrary(function) => {
                        if is_called_with_exclamation_mark {
                            return Err(Error::FunctionUnexpectedExclamationMark {
                                location: function_location.unwrap_or(location),
                                function: function.identifier().to_owned(),
                            });
                        }

                        let intrinsic_identifier = function.library_identifier();

                        let return_type =
                            function.call(function_location.unwrap_or(location), argument_list)?;

                        let element =
                            Value::try_from_type(&return_type, false, None).map(Element::Value)?;

                        let intermediate = GeneratorExpressionOperator::call_library(
                            intrinsic_identifier,
                            input_size,
                            return_type.size(),
                        );

                        (
                            element,
                            GeneratorExpressionElement::Operator {
                                location: function_location.unwrap_or(location),
                                operator: intermediate,
                            },
                        )
                    }
                }
            }
            FunctionType::Runtime(function) => {
                if is_called_with_exclamation_mark {
                    return Err(Error::FunctionUnexpectedExclamationMark {
                        location,
                        function: function.identifier,
                    });
                }

                let location = function.location;
                let type_id = function.type_id;

                let return_type = function.call(argument_list)?;

                let element =
                    Value::try_from_type(&return_type, false, None).map(Element::Value)?;

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
                if is_called_with_exclamation_mark {
                    return Err(Error::FunctionUnexpectedExclamationMark {
                        location,
                        function: function.identifier,
                    });
                }

                let arguments = function.validate(argument_list)?;

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
                return Err(Error::UnitTestCallForbidden {
                    location: function_location.unwrap_or(location),
                    function: function.identifier,
                });
            }
        };

        Ok((element, intermediate))
    }
}
