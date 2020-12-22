//!
//! The module semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_syntax::Module as SyntaxModule;
use zinc_syntax::ModuleLocalStatement;

use crate::semantic::analyzer::statement::module::Analyzer as ModStatementAnalyzer;
use crate::semantic::analyzer::statement::r#impl::Analyzer as ImplStatementAnalyzer;
use crate::semantic::analyzer::statement::r#use::Analyzer as UseStatementAnalyzer;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::statement::Statement as TypeStatementVariant;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
use crate::source::Source;

///
/// Analyzes a module, which is not the application entry point.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Declares the `module` with all the inner statements in the `scope`.
    ///
    /// `modules` contain the modules located in the directory of the module being analyzed.
    /// If the module is not a directory with `mod.zn`, but a standalone file, the dependency map
    /// is empty. Each module, declared using a `mod` statement, must have a corresponding file
    /// `<module>.zn` in the module directory. For example, `mod foo;` will look for a file called
    /// `./foo.zn` and yield an error if it is absent.
    ///
    /// Returns the module without the hoisted statements and the implementation scopes which
    /// must be defined forcibly.
    ///
    #[allow(clippy::type_complexity)]
    pub fn declare(
        scope: Rc<RefCell<Scope>>,
        mut module: SyntaxModule,
        mut modules: HashMap<String, Source>,
        scope_crate: Rc<RefCell<Scope>>,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
        is_entry: bool,
    ) -> Result<(SyntaxModule, Vec<Rc<RefCell<Scope>>>), Error> {
        let mut instant_statements = Vec::with_capacity(module.statements.len());
        let mut implementation_scopes = Vec::with_capacity(module.statements.len());

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
                    if !is_entry
                        && statement.identifier.name.as_str()
                            == zinc_const::source::FUNCTION_MAIN_IDENTIFIER
                    {
                        return Err(Error::FunctionMainBeyondEntry {
                            location: statement.location,
                        });
                    }

                    if is_entry
                        && statement.identifier.name.as_str()
                            == zinc_const::source::FUNCTION_MAIN_IDENTIFIER
                        && statement.is_constant
                    {
                        return Err(Error::EntryPointConstant {
                            location: statement.location,
                        });
                    }

                    Scope::declare_type(scope.clone(), TypeStatementVariant::Fn(statement))?;
                }
                ModuleLocalStatement::Mod(statement) => {
                    let module = match modules.remove(statement.identifier.name.as_str()) {
                        Some(module) => module,
                        None => {
                            return Err(Error::ModuleFileNotFound {
                                location: statement.identifier.location,
                                name: statement.identifier.name,
                            });
                        }
                    };

                    let identifier = ModStatementAnalyzer::analyze(statement)?;

                    Scope::declare_module(
                        scope.clone(),
                        identifier,
                        module,
                        scope_crate.clone(),
                        dependencies.clone(),
                    )?;
                }
                ModuleLocalStatement::Contract(statement) => {
                    if is_entry {
                        Scope::declare_contract(scope.clone(), statement)?;
                    } else {
                        return Err(Error::ContractBeyondEntry {
                            location: statement.location,
                        });
                    }
                }
                ModuleLocalStatement::Impl(statement) => {
                    let scope = ImplStatementAnalyzer::declare(scope.clone(), statement)?;
                    implementation_scopes.push(scope);
                }
                ModuleLocalStatement::Use(statement) => {
                    instant_statements.push(ModuleLocalStatement::Use(statement))
                }
                ModuleLocalStatement::Empty(_location) => {}
            }
        }

        module.statements = instant_statements;

        Ok((module, implementation_scopes))
    }

    ///
    /// 1. Defines the module aliases.
    /// 2. Defines the instant statements.
    /// 3. Resolves the implementation scopes forcibly.
    /// 4. Resolves the hoisted items forcibly.
    ///
    pub fn define(
        scope: Rc<RefCell<Scope>>,
        module: SyntaxModule,
        implementation_scopes: Vec<Rc<RefCell<Scope>>>,
        crate_item: Rc<RefCell<ScopeItem>>,
        super_item: Option<Rc<RefCell<ScopeItem>>>,
    ) -> Result<(), Error> {
        Scope::insert_item(scope.clone(), Keyword::Crate.to_string(), crate_item);
        if let Some(super_item) = super_item {
            Scope::insert_item(scope.clone(), Keyword::Super.to_string(), super_item);
        }

        for statement in module.statements.into_iter() {
            if let ModuleLocalStatement::Use(statement) = statement {
                UseStatementAnalyzer::define(scope.clone(), statement)?;
            }
        }

        for implementation_scope in implementation_scopes.into_iter() {
            implementation_scope.borrow().define()?;
        }

        scope.borrow().define()?;

        Ok(())
    }
}
