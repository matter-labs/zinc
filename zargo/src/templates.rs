//!
//! The Zargo generated files manifests.
//!

pub fn main_template(circuit_name: &str) -> String {
    format!(
        r#"//!
//! The '{}' main module.
//!

fn main(witness: u8) -> bool {{
    dbg!("Zello, World {{}}!", witness);

    true
}}
"#,
        circuit_name
    )
}
