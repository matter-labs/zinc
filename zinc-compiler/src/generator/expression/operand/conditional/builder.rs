//!
//! The generator expression conditional operand builder.
//!

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::operand::conditional::Expression as ConditionalExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use zinc_lexical::Location;

///
/// The generator expression conditional operand builder.
///
#[derive(Debug, Default, Clone)]
pub struct Builder {
    /// The conditional expression location.
    location: Option<Location>,
    /// The condition expression.
    condition: Option<GeneratorExpression>,
    /// The main block expression.
    main_block: Option<BlockExpression>,
    /// The `else`-block expression.
    else_block: Option<BlockExpression>,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, location: Location) {
        self.location = Some(location);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_condition(&mut self, value: GeneratorExpression) {
        self.condition = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_main_block(&mut self, value: BlockExpression) {
        self.main_block = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_else_block(&mut self, value: BlockExpression) {
        self.else_block = Some(value);
    }

    ///
    /// Finilizes the builder and returns the built item.
    ///
    pub fn finish(mut self) -> ConditionalExpression {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let condition = self.condition.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "condition"
            )
        });

        let main_block = self.main_block.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "main block"
            )
        });

        let else_block = self.else_block.take();

        ConditionalExpression::new(location, condition, main_block, else_block)
    }
}
