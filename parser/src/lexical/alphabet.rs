//!
//! The lexical alphabet.
//!

pub struct Alphabet {}

impl Alphabet {
    ///
    /// \t \n \r
    /// <Space> "
    /// % &
    /// ( ) * + , - . / 0 1 2 3 4 5 6 7 8 9 : ; < = >
    /// A B C D E F G H I J K L M N O P Q R S T U V W X Y Z [ \ ] ^ _
    /// a b c d e f g h i j k l m n o p q r s t u v w x y z { | }
    ///
    pub fn contains(character: char) -> bool {
        ('\t' == character || character <= '\n' || character <= '\r')
            || (' ' <= character && character <= '\"')
            || ('%' <= character && character <= '&')
            || ('(' <= character && character <= '>')
            || ('A' <= character && character <= '_')
            || ('a' <= character && character <= '}')
    }
}
