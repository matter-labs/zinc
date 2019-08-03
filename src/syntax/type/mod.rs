//!
//! The syntax type.
//!

mod keyword;

pub use self::keyword::Error as KeywordError;
pub use self::keyword::Keyword;

pub struct Type {
    keyword: Keyword,
}
