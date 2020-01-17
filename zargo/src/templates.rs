//!
//! The Zargo generated files manifests.
//!

pub fn manifest_template(circuit_name: &str) -> String {
    format!(
        r#"[circuit]
name = "{}"
version = "0.1.0"
"#,
        circuit_name
    )
}

pub fn main_template(circuit_name: &str) -> String {
    format!(
        r#"//!
//! The '{}' main module.
//!

fn main(input: (), witness: ()) -> u8 {{
    dbg!("Zello, World!");
    42
}}
"#,
        circuit_name
    )
}
