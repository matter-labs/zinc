//!
//! The syntax parser.
//!

mod error;
mod parser;
mod tree;

pub use self::error::Error;
pub use self::parser::Parser;
pub use self::tree::expression::array::Expression as ArrayExpression;
pub use self::tree::expression::auxiliary::Auxiliary as ExpressionAuxiliary;
pub use self::tree::expression::block::Expression as BlockExpression;
pub use self::tree::expression::conditional::Expression as ConditionalExpression;
pub use self::tree::expression::element::Element as ExpressionElement;
pub use self::tree::expression::object::Object as ExpressionObject;
pub use self::tree::expression::operand::Operand as ExpressionOperand;
pub use self::tree::expression::operator::Operator as ExpressionOperator;
pub use self::tree::expression::r#match::Expression as MatchExpression;
pub use self::tree::expression::structure::Expression as StructureExpression;
pub use self::tree::expression::tuple::Expression as TupleExpression;
pub use self::tree::expression::Expression;
pub use self::tree::field::Field;
pub use self::tree::identifier::Identifier;
pub use self::tree::literal::boolean::Literal as BooleanLiteral;
pub use self::tree::literal::integer::Literal as IntegerLiteral;
pub use self::tree::literal::string::Literal as StringLiteral;
pub use self::tree::member_integer::MemberInteger;
pub use self::tree::member_string::MemberString;
pub use self::tree::pattern_binding::variant::Variant as BindingPatternVariant;
pub use self::tree::pattern_binding::Pattern as BindingPattern;
pub use self::tree::pattern_match::variant::Variant as MatchPatternVariant;
pub use self::tree::pattern_match::Pattern as MatchPattern;
pub use self::tree::r#type::variant::Variant as TypeVariant;
pub use self::tree::r#type::Type;
pub use self::tree::statement::local_fn::Statement as FunctionLocalStatement;
pub use self::tree::statement::local_impl::Statement as ImplementationLocalStatement;
pub use self::tree::statement::local_mod::Statement as ModuleLocalStatement;
pub use self::tree::statement::module::Statement as ModStatement;
pub use self::tree::statement::r#const::Statement as ConstStatement;
pub use self::tree::statement::r#enum::Statement as EnumStatement;
pub use self::tree::statement::r#fn::Statement as FnStatement;
pub use self::tree::statement::r#impl::Statement as ImplStatement;
pub use self::tree::statement::r#let::Statement as LetStatement;
pub use self::tree::statement::r#loop::Statement as LoopStatement;
pub use self::tree::statement::r#struct::Statement as StructStatement;
pub use self::tree::statement::r#type::Statement as TypeStatement;
pub use self::tree::statement::r#use::Statement as UseStatement;
pub use self::tree::variant::Variant;
pub use self::tree::Tree;

static PANIC_BUILDER_REQUIRES_VALUE: &str = "The builder requires a value: ";
