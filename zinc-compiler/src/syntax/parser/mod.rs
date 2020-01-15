//!
//! The syntax parser.
//!

mod expression;
mod field;
mod field_list;
mod pattern_binding;
mod pattern_binding_list;
mod pattern_match;
mod statement;
mod r#type;
mod variant;
mod variant_list;

pub use self::expression::AccessOperandParser;
pub use self::expression::AddSubOperandParser;
pub use self::expression::AndOperandParser;
pub use self::expression::ArrayExpressionParser;
pub use self::expression::BlockExpressionParser;
pub use self::expression::CastingOperandParser;
pub use self::expression::ComparisonOperandParser;
pub use self::expression::ConditionalExpressionParser;
pub use self::expression::ListParser as ExpressionListParser;
pub use self::expression::MatchExpressionParser;
pub use self::expression::MulDivRemOperandParser;
pub use self::expression::OrOperandParser;
pub use self::expression::Parser as ExpressionParser;
pub use self::expression::PathOperandParser;
pub use self::expression::StructureExpressionParser;
pub use self::expression::TerminalOperandParser;
pub use self::expression::TupleExpressionParser;
pub use self::expression::XorOperandParser;
pub use self::field::Parser as FieldParser;
pub use self::field_list::Parser as FieldListParser;
pub use self::pattern_binding::Parser as BindingPatternParser;
pub use self::pattern_binding_list::Parser as BindingPatternListParser;
pub use self::pattern_match::Parser as MatchPatternParser;
pub use self::r#type::Parser as TypeParser;
pub use self::statement::ConstParser as ConstStatementParser;
pub use self::statement::EnumParser as EnumStatementParser;
pub use self::statement::ExternFnParser as ExternFnStatementParser;
pub use self::statement::FnParser as FnStatementParser;
pub use self::statement::FunctionLocalStatementParser;
pub use self::statement::ImplParser as ImplStatementParser;
pub use self::statement::ImplementationLocalStatementParser;
pub use self::statement::LetParser as LetStatementParser;
pub use self::statement::LoopParser as LoopStatementParser;
pub use self::statement::ModParser as ModStatementParser;
pub use self::statement::ModuleLocalStatementParser;
pub use self::statement::StaticParser as StaticStatementParser;
pub use self::statement::StructParser as StructStatementParser;
pub use self::statement::TypeParser as TypeStatementParser;
pub use self::statement::UseParser as UseStatementParser;
pub use self::variant::Parser as VariantParser;
pub use self::variant_list::Parser as VariantListParser;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::SyntaxTree;

#[derive(Default)]
pub struct Parser {
    next: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, input: String) -> Result<SyntaxTree, Error> {
        let stream = TokenStream::new(input);
        let stream = Rc::new(RefCell::new(stream));

        let mut statements = Vec::new();
        loop {
            match crate::syntax::take_or_next(self.next, stream.clone())? {
                Token {
                    lexeme: Lexeme::Eof,
                    ..
                } => break,
                token => {
                    let (statement, next) =
                        ModuleLocalStatementParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
                    log::trace!("Statement: {:?}", statement);
                    statements.push(statement);
                }
            }
        }

        Ok(SyntaxTree { statements })
    }
}

pub fn take_or_next(
    mut token: Option<Token>,
    stream: Rc<RefCell<TokenStream>>,
) -> Result<Token, Error> {
    match token.take() {
        Some(token) => Ok(token),
        None => Ok(stream.borrow_mut().next()?),
    }
}
