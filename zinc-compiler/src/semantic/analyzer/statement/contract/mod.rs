//!
//! The `contract` statement semantic analyzer.
//!

mod tests;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::statement::contract::Statement as GeneratorContractStatement;
use crate::semantic::analyzer::statement::field::Analyzer as FieldStatementAnalyzer;
use crate::semantic::analyzer::statement::r#fn::Context as FnStatementAnalyzerContext;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::statement::Statement as TypeStatementVariant;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::contract::Statement as ContractStatement;
use crate::syntax::tree::statement::local_contract::Statement as ContractLocalStatement;

///
/// The `contract` statement semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Initializes the contract type and declares the hoisted items.
    /// Removes the hoisted item statements from the contract statement.
    /// Returns the statement and allocated scope.
    ///
    pub fn declare(
        scope: Rc<RefCell<Scope>>,
        mut statement: ContractStatement,
    ) -> Result<(ContractStatement, Rc<RefCell<Scope>>), Error> {
        let mut instant_statements = Vec::with_capacity(statement.statements.len());
        for hoisted_statement in statement.statements.into_iter() {
            match hoisted_statement {
                ContractLocalStatement::Const(statement) => {
                    Scope::declare_constant(scope.clone(), statement, true)?;
                }
                ContractLocalStatement::Fn(statement) => {
                    Scope::declare_type(
                        scope.clone(),
                        TypeStatementVariant::Fn(statement, FnStatementAnalyzerContext::Contract),
                        true,
                    )?;
                }
                ContractLocalStatement::Empty(_location) => {}
                statement => instant_statements.push(statement),
            }
        }

        statement.statements = instant_statements;

        Ok((statement, scope))
    }

    ///
    /// Defines the instant items and forcibly defines the hoisted ones.
    ///
    pub fn define(
        scope: Rc<RefCell<Scope>>,
        statement: ContractStatement,
    ) -> Result<(Type, GeneratorContractStatement), Error> {
        let location = statement.location;

        let mut storage_fields = Vec::new();
        for instant_statement in statement.statements.into_iter() {
            if let ContractLocalStatement::Field(statement) = instant_statement {
                FieldStatementAnalyzer::define(
                    scope.clone(),
                    statement.clone(),
                    storage_fields.len(),
                )?;

                storage_fields.push((
                    statement.identifier.name.clone(),
                    Type::try_from_syntax(statement.r#type, scope.clone())?,
                ));
            }
        }

        let r#type = Type::contract(
            Some(statement.location),
            statement.identifier.name,
            storage_fields.clone(),
            Some(scope.clone()),
        );

        scope.borrow().define()?;

        let intermediate = GeneratorContractStatement::new(location, storage_fields);

        Ok((r#type, intermediate))
    }
}
