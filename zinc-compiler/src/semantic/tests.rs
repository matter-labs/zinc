//!
//! A semantic analyzer tests.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::semantic::analyzer::module::Analyzer as ModuleAnalyzer;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
use crate::source::file::index::INDEX as FILE_INDEX;
use crate::source::Source;
use crate::syntax::parser::Parser;
use crate::syntax::tree::module::Module as SyntaxModule;

pub(crate) fn compile_entry(code: &str) -> Result<(), Error> {
    compile_entry_with_dependencies(code, HashMap::new())
}

pub(crate) fn compile_entry_with_dependencies(
    code: &str,
    dependencies: HashMap<String, Source>,
) -> Result<(), Error> {
    let path = PathBuf::from("test.zn");
    EntryAnalyzer::define(Source::test(code, path, 0, dependencies)).map_err(Error::Semantic)?;

    Ok(())
}

pub(crate) fn compile_module(
    code: &str,
    file_index: usize,
    scope: Rc<RefCell<Scope>>,
    scope_crate: Rc<RefCell<Scope>>,
    scope_super: Rc<RefCell<Scope>>,
) -> Result<Rc<RefCell<Scope>>, Error> {
    compile_module_with_dependencies(
        code,
        file_index,
        scope,
        HashMap::new(),
        scope_crate,
        scope_super,
    )
}

pub(crate) fn compile_module_with_dependencies(
    code: &str,
    file_index: usize,
    scope: Rc<RefCell<Scope>>,
    dependencies: HashMap<String, Source>,
    scope_crate: Rc<RefCell<Scope>>,
    scope_super: Rc<RefCell<Scope>>,
) -> Result<Rc<RefCell<Scope>>, Error> {
    let module = Parser::default().parse(code, Some(file_index))?;
    let (module, implementation_scopes) = ModuleAnalyzer::declare(
        scope.clone(),
        module,
        dependencies,
        scope_crate.clone(),
        false,
    )?;

    let crate_item = Scope::get_module_self_alias(scope_crate);
    let super_item = Scope::get_module_self_alias(scope_super);

    ModuleAnalyzer::define(
        scope.clone(),
        module,
        implementation_scopes,
        crate_item,
        Some(super_item),
    )?;

    Ok(scope)
}

#[test]
fn error_entry_point_missing() {
    let code = r#"
fn another() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::EntryPointMissing));

    let result = crate::semantic::tests::compile_entry(code);

    assert_eq!(result, expected);
}

#[test]
fn error_entry_point_ambiguous() {
    let code = r#"
fn main() -> u8 {
    42
}

contract Uniswap {
    pub fn deposit(amount: u248) -> bool { true }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::EntryPointAmbiguous {
        main: Location::new(2, 1),
        contract: Location::new(6, 1),
    }));

    let result = crate::semantic::tests::compile_entry(code);

    assert_eq!(result, expected);
}

#[test]
fn error_entry_point_constant_function_main() {
    let code = r#"
const fn main() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::EntryPointConstant {
        location: Location::new(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry(code);

    assert_eq!(result, expected);
}

#[test]
fn error_entry_point_constant_contract_function() {
    let code = r#"
contract Uniswap {
    pub const fn deposit(amount: u248) -> bool { true }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::EntryPointConstant {
        location: Location::new(3, 5),
    }));

    let result = crate::semantic::tests::compile_entry(code);

    assert_eq!(result, expected);
}

#[test]
fn error_function_main_beyond_entry() {
    let code = r#"
fn main() -> u8 {
    42
}
"#;

    let expected = Error::Semantic(SemanticError::FunctionMainBeyondEntry {
        location: Location::new(2, 1),
    });

    let result = crate::semantic::tests::compile_module(
        code,
        0,
        Scope::new_global(zinc_const::source::APPLICATION_ENTRY_FILE_NAME.to_owned()).wrap(),
        Scope::new_global(zinc_const::source::APPLICATION_ENTRY_FILE_NAME.to_owned()).wrap(),
        Scope::new_global(zinc_const::source::APPLICATION_ENTRY_FILE_NAME.to_owned()).wrap(),
    )
    .unwrap_err();

    assert_eq!(result, expected);
}

#[test]
fn error_contract_beyond_entry() {
    let code = r#"
contract Uniswap {
    pub fn deposit(amount: u248) -> bool { true }
}
"#;

    let expected = Error::Semantic(SemanticError::ContractBeyondEntry {
        location: Location::new(2, 1),
    });

    let result = crate::semantic::tests::compile_module(
        code,
        0,
        Scope::new_global(zinc_const::source::APPLICATION_ENTRY_FILE_NAME.to_owned()).wrap(),
        Scope::new_global(zinc_const::source::APPLICATION_ENTRY_FILE_NAME.to_owned()).wrap(),
        Scope::new_global(zinc_const::source::APPLICATION_ENTRY_FILE_NAME.to_owned()).wrap(),
    )
    .unwrap_err();

    assert_eq!(result, expected);
}

#[test]
fn error_module_file_not_found() {
    let code = r#"
mod unknown;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ModuleFileNotFound {
        location: Location::new(2, 5),
        name: "unknown".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(code);

    assert_eq!(result, expected);
}
