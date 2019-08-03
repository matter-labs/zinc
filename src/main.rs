//!
//! The Jab compiler binary.
//!

use failure::Fail;

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Compiler: {}", _0)]
    Compiler(compiler::Error),
}

fn main() -> Result<(), Error> {
    let code = r#"
        inputs {
            a: uint8;
            b: field;
            c: bool;
        }

        witness {
            a: uint253;
            b: field;
            c: bool;
        }
    "#;

    let circuit = compiler::compile(&code).map_err(Error::Compiler)?;
    println!("{:?}", circuit);

    Ok(())
}
