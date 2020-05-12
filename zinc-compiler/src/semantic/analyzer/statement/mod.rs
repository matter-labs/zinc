//!
//! The statement semantic analyzer.
//!

pub mod r#const;
pub mod contract;
pub mod r#enum;
pub mod error;
pub mod field;
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
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::statement::Statement as TypeStatementVariant;
use crate::semantic::scope::Scope;
use crate::source::module::Module as SourceModule;
use crate::source::Source;
use crate::syntax::tree::statement::contract::Statement as ContractStatement;
use crate::syntax::tree::statement::local_contract::Statement as ContractLocalStatement;
use crate::syntax::tree::statement::local_fn::Statement as FunctionLocalStatement;
use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;
use crate::syntax::tree::statement::local_mod::Statement as ModuleLocalStatement;
use crate::syntax::tree::statement::r#impl::Statement as ImplementationStatement;

use self::field::Analyzer as FieldStatementAnalyzer;
use self::r#const::Analyzer as ConstStatementAnalyzer;
use self::r#fn::Context as FnStatementAnalyzerContext;
use self::r#for::Analyzer as ForStatementAnalyzer;
use self::r#impl::Analyzer as ImplStatementAnalyzer;
use self::r#let::Analyzer as LetStatementAnalyzer;
use self::r#use::Analyzer as UseStatementAnalyzer;

///
/// The context lets the analyzer know of file type where the analyzed statements are defined.
///
#[derive(Debug, Clone, Copy)]
pub enum Context {
    /// The circuit or contract entry file.
    Entry,
    /// An ordinar project module.
    Module,
}

///
/// Analyzes statements.
///
pub struct Analyzer {}

impl Analyzer {
    pub fn entry(source: Source, scope: Rc<RefCell<Scope>>) -> Result<(), Error> {
        let (module, mut dependencies) = (source.entry.tree, source.modules);

        for statement in module.statements.into_iter() {
            Self::local_module(statement, scope.clone(), Context::Entry, &mut dependencies)?;
        }

        scope.borrow().resolve()
    }

    pub fn module(module: SourceModule, scope: Rc<RefCell<Scope>>) -> Result<(), Error> {
        let (module, mut dependencies) = match module {
            SourceModule::File(file) => (file.tree, HashMap::new()),
            SourceModule::Directory(directory) => (directory.entry.tree, directory.modules),
        };

        for statement in module.statements.into_iter() {
            Self::local_module(statement, scope.clone(), Context::Module, &mut dependencies)?;
        }

        scope.borrow().resolve()
    }

    pub fn implementation(
        statement: ImplementationStatement,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<(), Error> {
        for statement in statement.statements.into_iter() {
            Self::local_implementation(statement, scope.clone())?;
        }

        scope.borrow().resolve()
    }

    pub fn contract(statement: ContractStatement, scope: Rc<RefCell<Scope>>) -> Result<(), Error> {
        for statement in statement.statements.into_iter() {
            Self::local_contract(statement, scope.clone())?;
        }

        scope.borrow().resolve()
    }

    ///
    /// Analyzes a statement local to a module.
    ///
    /// If the statement must be passed to the next compiler phase, yields its IR.
    ///
    pub fn local_module(
        statement: ModuleLocalStatement,
        scope: Rc<RefCell<Scope>>,
        context: Context,
        dependencies: &mut HashMap<String, SourceModule>,
    ) -> Result<(), Error> {
        match statement {
            ModuleLocalStatement::Const(statement) => {
                Scope::declare_constant(scope, statement)?;

                Ok(())
            }
            ModuleLocalStatement::Type(statement) => {
                Scope::declare_type(scope, TypeStatementVariant::Type(statement))?;

                Ok(())
            }
            ModuleLocalStatement::Struct(statement) => {
                Scope::declare_type(scope, TypeStatementVariant::Struct(statement))?;

                Ok(())
            }
            ModuleLocalStatement::Enum(statement) => {
                Scope::declare_type(scope, TypeStatementVariant::Enum(statement))?;

                Ok(())
            }
            ModuleLocalStatement::Fn(statement) => {
                if let Context::Module = context {
                    if statement.identifier.name.as_str() == crate::FUNCTION_MAIN_IDENTIFIER {
                        return Err(Error::FunctionMainBeyondEntry {
                            location: statement.location,
                        });
                    }
                }

                if let Context::Entry = context {
                    if statement.identifier.name.as_str() == crate::FUNCTION_MAIN_IDENTIFIER
                        && statement.is_constant
                    {
                        return Err(Error::EntryPointConstant {
                            location: statement.location,
                        });
                    }
                }

                Scope::declare_type(
                    scope,
                    TypeStatementVariant::Fn(statement, FnStatementAnalyzerContext::Module),
                )?;

                Ok(())
            }
            ModuleLocalStatement::Mod(statement) => {
                let module = match dependencies.remove(statement.identifier.name.as_str()) {
                    Some(module) => module,
                    None => {
                        return Err(Error::ModuleFileNotFound {
                            location: statement.identifier.location,
                            name: statement.identifier.name,
                        });
                    }
                };

                Scope::declare_module(scope, statement.identifier, module)?;

                Ok(())
            }
            ModuleLocalStatement::Use(statement) => {
                UseStatementAnalyzer::analyze(scope, statement)?;

                Ok(())
            }
            ModuleLocalStatement::Impl(statement) => {
                ImplStatementAnalyzer::analyze(scope, statement)?;

                Ok(())
            }
            ModuleLocalStatement::Contract(statement) => match context {
                Context::Entry => {
                    Scope::declare_contract(scope, statement)?;

                    Ok(())
                }
                Context::Module => Err(Error::ContractBeyondEntry {
                    location: statement.location,
                }),
            },
            ModuleLocalStatement::Empty(_location) => Ok(()),
        }
    }

    ///
    /// Analyzes a statement local to an implementation.
    ///
    /// If the statement must be passed to the next compiler phase, yields its IR.
    ///
    pub fn local_implementation(
        statement: ImplementationLocalStatement,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<(), Error> {
        match statement {
            ImplementationLocalStatement::Const(statement) => {
                let identifier = statement.identifier.clone();
                let constant = ConstStatementAnalyzer::analyze(scope.clone(), statement)?;
                Scope::define_constant(scope, identifier, constant)?;

                Ok(())
            }
            ImplementationLocalStatement::Fn(statement) => {
                Scope::declare_type(
                    scope,
                    TypeStatementVariant::Fn(statement, FnStatementAnalyzerContext::Implementation),
                )?;

                Ok(())
            }
            ImplementationLocalStatement::Empty(_location) => Ok(()),
        }
    }

    ///
    /// Analyzes a statement local to a contract.
    ///
    /// If the statement must be passed to the next compiler phase, yields its IR.
    ///
    pub fn local_contract(
        statement: ContractLocalStatement,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<(), Error> {
        match statement {
            ContractLocalStatement::Field(statement) => {
                FieldStatementAnalyzer::analyze(scope, statement)?;

                Ok(())
            }
            ContractLocalStatement::Const(statement) => {
                let identifier = statement.identifier.clone();
                let constant = ConstStatementAnalyzer::analyze(scope.clone(), statement)?;
                Scope::define_constant(scope, identifier, constant)?;

                Ok(())
            }
            ContractLocalStatement::Fn(statement) => {
                Scope::declare_type(
                    scope,
                    TypeStatementVariant::Fn(statement, FnStatementAnalyzerContext::Contract),
                )?;

                Ok(())
            }
            ContractLocalStatement::Empty(_location) => Ok(()),
        }
    }

    ///
    /// Analyzes a statement local to a function.
    ///
    /// If the statement must be passed to the next compiler phase, yields its IR.
    ///
    pub fn local_function(
        statement: FunctionLocalStatement,
        scope: Rc<RefCell<Scope>>,
        rule: TranslationRule,
    ) -> Result<Option<GeneratorStatement>, Error> {
        match statement {
            FunctionLocalStatement::Let(statement) => {
                let intermediate = LetStatementAnalyzer::analyze(scope.clone(), statement)?;
                Ok(intermediate.map(GeneratorStatement::Declaration))
            }
            FunctionLocalStatement::Const(statement) => {
                let identifier = statement.identifier.clone();
                let constant = ConstStatementAnalyzer::analyze(scope.clone(), statement)?;
                Scope::define_constant(scope, identifier, constant)?;

                Ok(None)
            }
            FunctionLocalStatement::For(statement) => {
                let intermediate = ForStatementAnalyzer::analyze(scope, statement)?;
                Ok(Some(GeneratorStatement::Loop(intermediate)))
            }
            FunctionLocalStatement::Expression(expression) => {
                let (_result, expression) =
                    ExpressionAnalyzer::new(scope, rule).analyze(expression)?;
                let intermediate = GeneratorStatement::Expression(expression);
                Ok(Some(intermediate))
            }
            FunctionLocalStatement::Empty(_location) => Ok(None),
        }
    }
}
