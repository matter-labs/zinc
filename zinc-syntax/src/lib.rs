//!
//! The Zinc syntax library.
//!

pub(crate) mod error;
pub(crate) mod parser;
pub(crate) mod tree;

pub use self::error::Error;
pub use self::error::ParsingError;
pub use self::parser::Parser;
pub use self::tree::attribute::element::variant::Variant as AttributeElementVariant;
pub use self::tree::attribute::element::Element as AttributeElement;
pub use self::tree::attribute::Attribute;
pub use self::tree::binding::Binding;
pub use self::tree::expression::array::variant::Variant as ArrayExpressionVariant;
pub use self::tree::expression::array::Expression as ArrayExpression;
pub use self::tree::expression::block::Expression as BlockExpression;
pub use self::tree::expression::conditional::Expression as ConditionalExpression;
pub use self::tree::expression::list::Expression as ListExpression;
pub use self::tree::expression::r#match::Expression as MatchExpression;
pub use self::tree::expression::structure::Expression as StructureExpression;
pub use self::tree::expression::tree::node::operand::Operand as ExpressionOperand;
pub use self::tree::expression::tree::node::operator::Operator as ExpressionOperator;
pub use self::tree::expression::tree::node::Node as ExpressionTreeNode;
pub use self::tree::expression::tree::Tree as ExpressionTree;
pub use self::tree::expression::tuple::Expression as TupleExpression;
pub use self::tree::identifier::Identifier;
pub use self::tree::literal::boolean::Literal as BooleanLiteral;
pub use self::tree::literal::integer::Literal as IntegerLiteral;
pub use self::tree::literal::string::Literal as StringLiteral;
pub use self::tree::literal::Literal;
pub use self::tree::module::Module;
pub use self::tree::pattern_binding::variant::Variant as BindingPatternVariant;
pub use self::tree::pattern_binding::Pattern as BindingPattern;
pub use self::tree::pattern_match::variant::Variant as MatchPatternVariant;
pub use self::tree::pattern_match::Pattern as MatchPattern;
pub use self::tree::r#type::variant::Variant as TypeVariant;
pub use self::tree::r#type::Type;
pub use self::tree::statement::contract::Statement as ContractStatement;
pub use self::tree::statement::field::Statement as FieldStatement;
pub use self::tree::statement::local_contract::Statement as ContractLocalStatement;
pub use self::tree::statement::local_fn::Statement as FunctionLocalStatement;
pub use self::tree::statement::local_impl::Statement as ImplementationLocalStatement;
pub use self::tree::statement::local_mod::Statement as ModuleLocalStatement;
pub use self::tree::statement::module::Statement as ModStatement;
pub use self::tree::statement::r#const::Statement as ConstStatement;
pub use self::tree::statement::r#enum::Statement as EnumStatement;
pub use self::tree::statement::r#fn::Statement as FnStatement;
pub use self::tree::statement::r#for::Statement as ForStatement;
pub use self::tree::statement::r#impl::Statement as ImplStatement;
pub use self::tree::statement::r#let::Statement as LetStatement;
pub use self::tree::statement::r#struct::Statement as StructStatement;
pub use self::tree::statement::r#type::Statement as TypeStatement;
pub use self::tree::statement::r#use::Statement as UseStatement;
pub use self::tree::tuple_index::TupleIndex;
pub use self::tree::variant::Variant;
