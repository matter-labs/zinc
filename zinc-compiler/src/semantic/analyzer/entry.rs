//!
//! The entry point semantic analyzer.
//!

use std::cell::RefCell;
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
    /// 1. Defines the entry module aliases.
    /// 2. Calls the module statements analyzer.
    /// 3. Defines the module items forcibly.
    /// 4. Validates entry points.
    ///
    pub fn define(module: Source) -> Result<Rc<RefCell<Scope>>, Error> {
        let entry = ScopeModuleItem::new_entry(module)?;
        entry.borrow().define()?;

        let entry = entry.borrow();
        if let ScopeItem::Module(ref module) = *entry {
            let scope = module.scope()?;

            let main_function_location = scope.borrow().get_main_location();
            let contract_location = scope.borrow().get_contract_location();

            if main_function_location.is_none() && contract_location.is_none() {
                return Err(Error::EntryPointMissing);
            }

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
            panic!(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);
        }
    }
}
