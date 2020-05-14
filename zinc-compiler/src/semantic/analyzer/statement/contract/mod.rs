//!
//! The `contract` statement semantic analyzer.
//!

mod tests;

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::analyzer::statement::field::Analyzer as FieldStatementAnalyzer;
use crate::semantic::analyzer::statement::r#fn::Context as FnStatementAnalyzerContext;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::statement::Statement as TypeStatementVariant;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::contract::Statement as ContractStatement;
use crate::syntax::tree::statement::local_contract::Statement as ContractLocalStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only contract declaration statement.
    /// Works in four phases:
    ///
    /// 1. Initializes the contract type.
    /// 2. Declares the hoisted items.
    /// 3. Defines the instant items.
    /// 4. Resolves the hoisted items forcibly.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: ContractStatement) -> Result<Type, Error> {
        let scope = Scope::new_child(scope);

        let r#type = Type::contract(
            Some(statement.location),
            statement.identifier.name.clone(),
            Some(scope.clone()),
        );

        let mut instant_statements = Vec::with_capacity(statement.statements.len());
        for hoisted_statement in statement.statements.into_iter() {
            match hoisted_statement {
                ContractLocalStatement::Const(statement) => {
                    Scope::declare_constant(scope.clone(), statement)?;
                }
                ContractLocalStatement::Fn(statement) => {
                    Scope::declare_type(
                        scope.clone(),
                        TypeStatementVariant::Fn(statement, FnStatementAnalyzerContext::Contract),
                    )?;
                }
                ContractLocalStatement::Empty(_location) => {}
                statement => instant_statements.push(statement),
            }
        }

        let mut contract_field_index = 0;
        for instant_statement in instant_statements.into_iter() {
            if let ContractLocalStatement::Field(statement) = instant_statement {
                FieldStatementAnalyzer::analyze(scope.clone(), statement, contract_field_index)?;
                contract_field_index += 1;
            }
        }

        scope.borrow().resolve()?;

        Ok(r#type)
    }
}
