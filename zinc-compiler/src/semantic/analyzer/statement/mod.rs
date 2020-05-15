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
use crate::syntax::tree::module::Module as SyntaxModule;
use crate::syntax::tree::statement::local_fn::Statement as FunctionLocalStatement;
use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;
use crate::syntax::tree::statement::local_mod::Statement as ModuleLocalStatement;
use crate::syntax::tree::statement::r#impl::Statement as ImplementationStatement;

use self::module::Analyzer as ModStatementAnalyzer;
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
    ///
    /// Analyzes the module or entry with all the inner statements. Works in three phases:
    ///
    /// 1. Declares the hoisted items.
    /// 2. Defines the instant items.
    /// 3. Resolves the hoisted items forcibly.
    ///
    /// `dependencies` contain the modules located in the directory of the module being analyzed.
    /// If the module is not a directory with `mod.zn`, but a standalone file, the dependency map
    /// is empty. Each module, declared using a `mod` statement, must have a corresponding file
    /// `<module>.zn` in the module directory. For example, `mod foo;` will look for file called
    /// `./foo.zn` and yield an error if it is absent.
    ///
    /// `context` specifies whether the current module in the application entry point or an ordinar
    /// module found elsewhere.
    ///
    pub fn module(
        module: SyntaxModule,
        mut dependencies: HashMap<String, SourceModule>,
        context: Context,
    ) -> Result<Rc<RefCell<Scope>>, Error> {
        let scope = Scope::new_global().wrap();

        let mut instant_statements = Vec::with_capacity(module.statements.len());
        for hoisted_statement in module.statements.into_iter() {
            match hoisted_statement {
                ModuleLocalStatement::Const(statement) => {
                    Scope::declare_constant(scope.clone(), statement)?;
                }
                ModuleLocalStatement::Type(statement) => {
                    Scope::declare_type(scope.clone(), TypeStatementVariant::Type(statement))?;
                }
                ModuleLocalStatement::Struct(statement) => {
                    Scope::declare_type(scope.clone(), TypeStatementVariant::Struct(statement))?;
                }
                ModuleLocalStatement::Enum(statement) => {
                    Scope::declare_type(scope.clone(), TypeStatementVariant::Enum(statement))?;
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
                        scope.clone(),
                        TypeStatementVariant::Fn(statement, FnStatementAnalyzerContext::Module),
                    )?;
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

                    let identifier = ModStatementAnalyzer::analyze(statement)?;

                    Scope::declare_module(scope.clone(), identifier, module)?;
                }
                ModuleLocalStatement::Contract(statement) => match context {
                    Context::Entry => {
                        Scope::declare_contract(scope.clone(), statement)?;
                    }
                    Context::Module => {
                        return Err(Error::ContractBeyondEntry {
                            location: statement.location,
                        })
                    }
                },
                ModuleLocalStatement::Empty(_location) => {}
                statement => instant_statements.push(statement),
            }
        }

        for instant_statement in instant_statements.into_iter() {
            match instant_statement {
                ModuleLocalStatement::Use(statement) => {
                    UseStatementAnalyzer::analyze(scope.clone(), statement)?;
                }
                ModuleLocalStatement::Impl(statement) => {
                    ImplStatementAnalyzer::analyze(scope.clone(), statement)?;
                }
                _ => {}
            }
        }

        scope.borrow().resolve()?;
        Ok(scope)
    }

    ///
    /// Analyzes the `impl` statement with all the inner statements. Works in three phases:
    ///
    /// 1. Declares the hoisted items.
    /// 2. Defines the instant items.
    /// 3. Resolves the hoisted items forcibly.
    ///
    pub fn implementation(
        statement: ImplementationStatement,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<(), Error> {
        for hoisted_statement in statement.statements.into_iter() {
            match hoisted_statement {
                ImplementationLocalStatement::Const(statement) => {
                    Scope::declare_constant(scope.clone(), statement)?;
                }
                ImplementationLocalStatement::Fn(statement) => {
                    Scope::declare_type(
                        scope.clone(),
                        TypeStatementVariant::Fn(
                            statement,
                            FnStatementAnalyzerContext::Implementation,
                        ),
                    )?;
                }
                ImplementationLocalStatement::Empty(_location) => {}
            }
        }

        scope.borrow().resolve()
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
                let intermediate = LetStatementAnalyzer::analyze(scope, statement)?;
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
