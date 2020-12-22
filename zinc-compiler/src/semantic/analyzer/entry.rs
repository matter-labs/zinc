//!
//! The entry point semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::semantic::error::Error;
use crate::semantic::scope::item::module::Module as ScopeModuleItem;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
use crate::source::Source;

///
/// Analyzes the application entry.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Forcibly and recursively defines the entry module.
    ///
    pub fn define(
        module: Source,
        project: zinc_project::ManifestProject,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
        is_dependency_entry: bool,
    ) -> Result<Rc<RefCell<Scope>>, Error> {
        let entry = ScopeModuleItem::new_entry(module, project, dependencies, is_dependency_entry)?;
        entry.borrow().define()?;

        let entry = entry.borrow();
        if let ScopeItem::Module(ref module) = *entry {
            let scope = module.scope()?;

            let main_function_location = scope.borrow().get_main_location();
            let contract_location = scope.borrow().get_contract_location();

            if let (Some(main_location), Some(contract_location)) =
                (main_function_location, contract_location)
            {
                return Err(Error::EntryPointAmbiguous {
                    main: main_location,
                    contract: contract_location,
                });
            }

            Ok(scope)
        } else {
            panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);
        }
    }
}
