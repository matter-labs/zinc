//!
//! The entry point semantic analyzer.
//!

use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::source::Source;

///
/// Analyzes the project entry, which must be located in the `main.zn` file.
///
/// To analyze a project module, use the module analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    pub fn analyze(source: Source) -> Result<(), Error> {
        let scope = Scope::new_global().wrap();

        StatementAnalyzer::entry(source, scope.clone())?;

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

        Ok(())
    }
}
