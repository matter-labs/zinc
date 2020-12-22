//!
//! A semantic analyzer tests.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use zinc_lexical::Location;
use zinc_syntax::Parser;

use crate::error::Error;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::semantic::analyzer::module::Analyzer as ModuleAnalyzer;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::Scope;
use crate::source::Source;

pub(crate) fn compile_entry(code: &str) -> Result<(), Error> {
    compile_entry_with_modules(code, HashMap::new())
}

pub(crate) fn compile_entry_with_modules(
    code: &str,
    modules: HashMap<String, Source>,
) -> Result<(), Error> {
    let path = PathBuf::from("test.zn");
    let source = Source::test(code, path, modules).expect(zinc_const::panic::TEST_DATA_VALID);
    let project = zinc_project::ManifestProject::new(
        "test".to_owned(),
        zinc_project::ProjectType::Contract,
        semver::Version::new(1, 0, 0),
    );

    EntryAnalyzer::define(source, project, HashMap::new(), false).map_err(Error::Semantic)?;

    Ok(())
}

pub(crate) fn compile_module(
    code: &str,
    file: usize,
    scope: Rc<RefCell<Scope>>,
    scope_crate: Rc<RefCell<Scope>>,
    scope_super: Rc<RefCell<Scope>>,
) -> Result<Rc<RefCell<Scope>>, Error> {
    compile_module_with_modules(code, file, scope, HashMap::new(), scope_crate, scope_super)
}

pub(crate) fn compile_module_with_modules(
    code: &str,
    file: usize,
    scope: Rc<RefCell<Scope>>,
    modules: HashMap<String, Source>,
    scope_crate: Rc<RefCell<Scope>>,
    scope_super: Rc<RefCell<Scope>>,
) -> Result<Rc<RefCell<Scope>>, Error> {
    let module = Parser::default().parse(code, file)?;
    let (module, implementation_scopes) = ModuleAnalyzer::declare(
        scope.clone(),
        module,
        modules,
        scope_crate.clone(),
        HashMap::new(),
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
        main: Location::test(2, 1),
        contract: Location::test(6, 1),
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
        location: Location::test(2, 1),
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
        location: Location::test(3, 5),
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
        location: Location::test(2, 1),
    });

    let scope = Scope::new_module(
        zinc_const::file_name::APPLICATION_ENTRY.to_owned(),
        HashMap::new(),
        None,
        false,
    )
    .wrap();
    let result =
        crate::semantic::tests::compile_module(code, 0, scope.clone(), scope.clone(), scope)
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
        location: Location::test(2, 1),
    });

    let scope = Scope::new_module(
        zinc_const::file_name::APPLICATION_ENTRY.to_owned(),
        HashMap::new(),
        None,
        false,
    )
    .wrap();
    let result =
        crate::semantic::tests::compile_module(code, 0, scope.clone(), scope.clone(), scope)
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
        location: Location::test(2, 5),
        name: "unknown".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(code);

    assert_eq!(result, expected);
}
