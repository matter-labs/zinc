//!
//! The statement semantic analyzer.
//!

pub mod r#const;
pub mod contract;
pub mod r#enum;
pub mod error;
pub mod r#fn;
pub mod r#for;
pub mod r#impl;
pub mod r#let;
pub mod module;
pub mod r#struct;
pub mod r#type;
pub mod r#use;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::error::Error;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::local_fn::Statement as FunctionLocalStatement;
use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;
use crate::syntax::tree::statement::local_mod::Statement as ModLocalStatement;

use self::contract::Analyzer as ContractStatementAnalyzer;
use self::module::Analyzer as ModStatementAnalyzer;
use self::r#const::Analyzer as ConstStatementAnalyzer;
use self::r#enum::Analyzer as EnumStatementAnalyzer;
use self::r#fn::Analyzer as FnStatementAnalyzer;
use self::r#for::Analyzer as ForStatementAnalyzer;
use self::r#impl::Analyzer as ImplStatementAnalyzer;
use self::r#let::Analyzer as LetStatementAnalyzer;
use self::r#struct::Analyzer as StructStatementAnalyzer;
use self::r#type::Analyzer as TypeStatementAnalyzer;
use self::r#use::Analyzer as UseStatementAnalyzer;

///
/// The context lets the analyzer know of file type where the analyzed statements are defined.
///
pub enum Context {
    /// The circuit or contract entry file.
    Entry,
    /// An ordinar project module.
    Module,
}

///
/// Analyzes statements.
///
/// An analyzer instance can be reused to analyze statements located in the same item, e.g. in the
/// same module, function, or implementation.
///
pub struct Analyzer {
    scope_stack: ScopeStack,
    dependencies: HashMap<String, Rc<RefCell<Scope>>>,
}

impl Analyzer {
    pub fn new(
        scope: Rc<RefCell<Scope>>,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Self {
        Self {
            scope_stack: ScopeStack::new(scope),
            dependencies,
        }
    }

    ///
    /// Analyzes a statement local to a module.
    ///
    /// If the statement must be passed to the next compiler phase, yields its IR.
    ///
    pub fn local_mod(
        &mut self,
        statement: ModLocalStatement,
        context: Context,
    ) -> Result<Option<GeneratorStatement>, Error> {
        match statement {
            ModLocalStatement::Const(statement) => {
                ConstStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(None)
            }
            ModLocalStatement::Type(statement) => {
                TypeStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(None)
            }
            ModLocalStatement::Struct(statement) => {
                StructStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(None)
            }
            ModLocalStatement::Enum(statement) => {
                EnumStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(None)
            }
            ModLocalStatement::Fn(statement) => {
                let intermediate = FnStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(Some(GeneratorStatement::Function(intermediate)))
            }
            ModLocalStatement::Mod(statement) => {
                ModStatementAnalyzer::analyze(
                    self.scope_stack.top(),
                    statement,
                    &mut self.dependencies,
                )?;
                Ok(None)
            }
            ModLocalStatement::Use(statement) => {
                UseStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(None)
            }
            ModLocalStatement::Impl(statement) => {
                let intermediate =
                    ImplStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(Some(GeneratorStatement::Implementation(intermediate)))
            }
            ModLocalStatement::Contract(statement) => match context {
                Context::Entry => {
                    let intermediate =
                        ContractStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                    Ok(Some(GeneratorStatement::Contract(intermediate)))
                }
                Context::Module => Err(Error::ContractBeyondEntry),
            },
            ModLocalStatement::Empty(_location) => Ok(None),
        }
    }

    ///
    /// Analyzes a statement local to a function.
    ///
    /// If the statement must be passed to the next compiler phase, yields its IR.
    ///
    pub fn local_fn(
        &mut self,
        statement: FunctionLocalStatement,
    ) -> Result<Option<GeneratorStatement>, Error> {
        match statement {
            FunctionLocalStatement::Let(statement) => {
                let intermediate =
                    LetStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(intermediate.map(GeneratorStatement::Declaration))
            }
            FunctionLocalStatement::Const(statement) => {
                ConstStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(None)
            }
            FunctionLocalStatement::For(statement) => {
                let intermediate =
                    ForStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(Some(GeneratorStatement::Loop(intermediate)))
            }
            FunctionLocalStatement::Expression(expression) => {
                let (_result, expression) = ExpressionAnalyzer::new(self.scope_stack.top())
                    .analyze(expression, TranslationHint::Value)?;
                let intermediate = GeneratorStatement::Expression(expression);
                Ok(Some(intermediate))
            }
            FunctionLocalStatement::Empty(_location) => Ok(None),
        }
    }

    ///
    /// Analyzes a statement local to an implementation.
    ///
    /// If the statement must be passed to the next compiler phase, yields its IR.
    ///
    pub fn local_impl(
        &mut self,
        statement: ImplementationLocalStatement,
    ) -> Result<Option<GeneratorStatement>, Error> {
        match statement {
            ImplementationLocalStatement::Const(statement) => {
                ConstStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(None)
            }
            ImplementationLocalStatement::Fn(statement) => {
                let intermediate = FnStatementAnalyzer::analyze(self.scope_stack.top(), statement)?;
                Ok(Some(GeneratorStatement::Function(intermediate)))
            }
            ImplementationLocalStatement::Empty(_location) => Ok(None),
        }
    }
}
