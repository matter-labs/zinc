//!
//! Transpiler output.
//!

mod array;
mod intermediate;
mod statements;
mod structure;
mod tuple;
mod r#type;
mod variable;

pub use self::array::Output as ArrayOutput;
pub use self::intermediate::AdditionOutput as IntermediateAdditionOutput;
pub use self::intermediate::AndOutput as IntermediateAndOutput;
pub use self::intermediate::AssignmentOutput as IntermediateAssignmentOutput;
pub use self::intermediate::CastingOutput as IntermediateCastingOutput;
pub use self::intermediate::DivisionOutput as IntermediateDivisionOutput;
pub use self::intermediate::EqualsOutput as IntermediateEqualsOutput;
pub use self::intermediate::GreaterEqualsOutput as IntermediateGreaterEqualsOutput;
pub use self::intermediate::GreaterOutput as IntermediateGreaterOutput;
pub use self::intermediate::LesserEqualsOutput as IntermediateLesserEqualsOutput;
pub use self::intermediate::LesserOutput as IntermediateLesserOutput;
pub use self::intermediate::MultiplicationOutput as IntermediateMultiplicationOutput;
pub use self::intermediate::NegationOutput as IntermediateNegationOutput;
pub use self::intermediate::NotEqualsOutput as IntermediateNotEqualsOutput;
pub use self::intermediate::NotOutput as IntermediateNotOutput;
pub use self::intermediate::OrOutput as IntermediateOrOutput;
pub use self::intermediate::RemainderOutput as IntermediateRemainderOutput;
pub use self::intermediate::SubtractionOutput as IntermediateSubtractionOutput;
pub use self::intermediate::XorOutput as IntermediateXorOutput;
pub use self::r#type::Output as TypeOutput;
pub use self::statements::DebugOutput as DebugStatementOutput;
pub use self::statements::LetOutput as LetStatementOutput;
pub use self::statements::RequireOutput as RequireStatementOutput;
pub use self::statements::StructOutput as StructStatementOutput;
pub use self::statements::TypeOutput as TypeStatementOutput;
pub use self::structure::Output as StructureOutput;
pub use self::tuple::Output as TupleOutput;
pub use self::variable::Output as VariableOutput;
