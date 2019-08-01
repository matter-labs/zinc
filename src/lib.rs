//pub mod examples;
//pub mod gadgets;

mod syntax;

extern crate proc_macro;

use proc_macro::TokenStream;

use self::syntax::Analyzer;

#[proc_macro]
pub fn circuit(input: TokenStream) -> TokenStream {
    match Analyzer::new().analyze(input) {
        Ok(()) => println!("Syntax analysis successful!"),
        Err(error) => panic!("Syntax error: {}", error),
    }

    TokenStream::new()
}
