//!
//! The semantic analyzer contract type element.
//!

#[cfg(test)]
mod tests;

pub mod field;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Location;
use zinc_syntax::Identifier;

use crate::semantic::element::r#type::function::intrinsic::contract_fetch::Function as ContractFetchFunction;
use crate::semantic::element::r#type::function::intrinsic::contract_transfer::Function as ContractTransferFunction;
use crate::semantic::element::r#type::function::intrinsic::Function as IntrinsicFunction;
use crate::semantic::element::r#type::Function;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;

use self::field::Field;

///
/// Describes a contract type.
///
/// Consists of the local contract `identifier` within its scope, global `type_id`,
/// and the implementation `scope`, which contains the reference to its parent scope.
///
#[derive(Debug, Clone)]
pub struct Contract {
    /// The contract type location in the code.
    pub location: Location,
    /// The contract type identifier.
    pub identifier: String,
    /// The `project` section of the contract project manifest.
    pub project: zinc_project::ManifestProject,
    /// The unique contract type ID.
    pub type_id: usize,
    /// The ordered contract storage fields array.
    pub fields: Vec<Field>,
    /// The contract scope, where its methods and associated items are declared.
    pub scope: Rc<RefCell<Scope>>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Location,
        identifier: String,
        project: zinc_project::ManifestProject,
        type_id: usize,
        fields: Vec<Field>,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Self, Error> {
        Scope::define_field(
            scope.clone(),
            Identifier::new(
                location,
                zinc_const::contract::FIELD_NAME_ADDRESS.to_owned(),
            ),
            Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS),
            zinc_const::contract::FIELD_INDEX_ADDRESS,
            true,
            true,
            true,
        )?;
        Scope::define_field(
            scope.clone(),
            Identifier::new(
                location,
                zinc_const::contract::FIELD_NAME_BALANCES.to_owned(),
            ),
            Scope::resolve_mtreemap(location, scope.clone()),
            zinc_const::contract::FIELD_INDEX_BALANCES,
            true,
            true,
            true,
        )?;

        let contract = Self {
            location,
            identifier,
            project,
            type_id,
            fields,
            scope: scope.clone(),
        };

        Scope::define_type(
            scope.clone(),
            Identifier::new(location, ContractFetchFunction::IDENTIFIER.to_owned()),
            Type::Function(Function::Intrinsic(IntrinsicFunction::contract_fetch(
                contract.clone(),
            ))),
            None,
        )?;
        Scope::define_type(
            scope.clone(),
            Identifier::new(location, ContractTransferFunction::IDENTIFIER.to_owned()),
            Type::Function(Function::Intrinsic(IntrinsicFunction::ContractTransfer(
                ContractTransferFunction::default(),
            ))),
            None,
        )?;

        Scope::insert_item(
            scope,
            Keyword::SelfUppercase.to_string(),
            ScopeItem::Type(ScopeTypeItem::new_defined(
                Some(location),
                Type::Contract(contract.clone()),
                true,
                None,
            ))
            .wrap(),
        );

        Ok(contract)
    }
}

impl PartialEq<Self> for Contract {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
