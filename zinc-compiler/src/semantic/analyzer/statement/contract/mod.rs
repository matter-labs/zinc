//!
//! The `contract` statement semantic analyzer.
//!

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::ContractLocalStatement;
use zinc_syntax::ContractStatement;
use zinc_syntax::Identifier;

use crate::generator::statement::contract::Statement as GeneratorContractStatement;
use crate::semantic::analyzer::statement::field::Analyzer as FieldStatementAnalyzer;
use crate::semantic::element::r#type::contract::field::Field as ContractFieldType;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::statement::Statement as TypeStatementVariant;
use crate::semantic::scope::Scope;

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
                    Scope::declare_constant(scope.clone(), statement)?;
                }
                ContractLocalStatement::Fn(statement) => {
                    Scope::declare_type(scope.clone(), TypeStatementVariant::Fn(statement))?;
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

        let mut storage_fields = Vec::with_capacity(zinc_const::contract::IMPLICIT_FIELDS_COUNT);
        storage_fields.insert(
            zinc_const::contract::FIELD_INDEX_ADDRESS,
            ContractFieldType::new(
                Identifier::new(
                    statement.location,
                    zinc_const::contract::FIELD_NAME_ADDRESS.to_owned(),
                ),
                Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS),
                true,
                true,
                true,
            ),
        );
        storage_fields.insert(
            zinc_const::contract::FIELD_INDEX_BALANCES,
            ContractFieldType::new(
                Identifier::new(
                    statement.location,
                    zinc_const::contract::FIELD_NAME_BALANCES.to_owned(),
                ),
                Scope::resolve_mtreemap(statement.location, scope.clone()),
                true,
                true,
                true,
            ),
        );

        for instant_statement in statement.statements.into_iter() {
            if let ContractLocalStatement::Field(statement) = instant_statement {
                FieldStatementAnalyzer::define(
                    scope.clone(),
                    statement.clone(),
                    storage_fields.len(),
                )?;

                let field = ContractFieldType::try_from_syntax(statement, scope.clone())?;

                storage_fields.push(field);
            }
        }

        let (project, is_in_dependency) =
            RefCell::borrow(&scope)
                .entry()
                .ok_or(Error::ContractBeyondEntry {
                    location: statement.location,
                })?;

        let r#type = Type::contract(
            statement.location,
            statement.identifier.name,
            project.clone(),
            storage_fields.clone(),
            scope.clone(),
        )?;

        scope.borrow().define()?;

        let intermediate =
            GeneratorContractStatement::new(location, project, storage_fields, is_in_dependency);

        Ok((r#type, intermediate))
    }
}
