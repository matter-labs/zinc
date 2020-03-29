//!
//! The function call semantic analyzer.
//!

use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::generator::expression::operator::Operator as GeneratorExpressionOperator;
use crate::lexical::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::builtin::Function as BuiltInFunctionType;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::Variant as ScopeItemVariant;
use crate::semantic::scope::Scope;

pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        operand_1: Element,
        operand_2: Element,
        is_call_builtin: bool,
        location: Location,
    ) -> Result<(Element, GeneratorExpressionOperator), Error> {
        let function = match operand_1 {
            Element::Type(Type::Function(function)) => function,
            Element::Path(path) => match Scope::resolve_path(scope.clone(), &path)?.variant {
                ScopeItemVariant::Type(Type::Function(function)) => function,
                item => {
                    return Err(Error::Function(
                        location,
                        FunctionError::non_callable(item.to_string()),
                    ));
                }
            },
            operand => {
                return Err(Error::Function(
                    location,
                    FunctionError::non_callable(operand.to_string()),
                ));
            }
        };

        let argument_elements = match operand_2 {
            Element::ArgumentList(values) => values,
            _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
        };
        let mut input_size = 0;
        for element in argument_elements.iter() {
            input_size += Type::from_element(element, scope.clone())?.size();
        }

        let (return_type, intermediate) = match function {
            FunctionType::BuiltInFunction(function) => {
                if !is_call_builtin {
                    return Err(Error::Function(
                        location,
                        FunctionError::BuiltIn(BuiltInFunctionError::specifier_missing(
                            function.identifier(),
                        )),
                    ));
                }

                match function {
                    BuiltInFunctionType::Debug(function) => {
                        let (return_type, format, argument_types) = function
                            .call(argument_elements)
                            .map_err(|error| Error::Function(location, error))?;

                        let intermediate =
                            GeneratorExpressionOperator::call_debug(format, argument_types);

                        (return_type, intermediate)
                    }
                    BuiltInFunctionType::Assert(function) => {
                        let (return_type, message) = function
                            .call(argument_elements)
                            .map_err(|error| Error::Function(location, error))?;

                        let intermediate = GeneratorExpressionOperator::call_assert(message);

                        (return_type, intermediate)
                    }
                }
            }
            FunctionType::StandardLibrary(function) => {
                if is_call_builtin {
                    return Err(Error::Function(
                        location,
                        FunctionError::BuiltIn(BuiltInFunctionError::unknown(
                            function.identifier().to_owned(),
                        )),
                    ));
                }

                let builtin_identifier = function.builtin_identifier();

                let return_type = function
                    .call(argument_elements)
                    .map_err(|error| Error::Function(location, error))?;

                let intermediate = GeneratorExpressionOperator::call_std(
                    builtin_identifier,
                    input_size,
                    return_type.size(),
                );

                (return_type, intermediate)
            }
            FunctionType::UserDefined(function) => {
                if is_call_builtin {
                    return Err(Error::Function(
                        location,
                        FunctionError::BuiltIn(BuiltInFunctionError::unknown(
                            function.identifier().to_owned(),
                        )),
                    ));
                }

                let unique_id = function.unique_id();

                let return_type = function
                    .call(argument_elements)
                    .map_err(|error| Error::Function(location, error))?;

                let intermediate = GeneratorExpressionOperator::call(unique_id, input_size);

                (return_type, intermediate)
            }
        };

        let element = Element::Value(
            Value::try_from(&return_type)
                .map_err(ElementError::Value)
                .map_err(|error| Error::Element(location, error))?,
        );

        Ok((element, intermediate))
    }
}
