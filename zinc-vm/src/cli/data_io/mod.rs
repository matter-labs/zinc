pub mod circuit_format;

use json::JsonValue;
use num_bigint::BigInt;

use num_traits::Num;

/// Choose radix automatically.
fn bigint_from_str(string: &str) -> Option<BigInt> {
    if string.starts_with("0x") {
        BigInt::from_str_radix(&string[2..], 16).ok()
    } else {
        BigInt::from_str_radix(string, 10).ok()
    }
}

/// Walks through json structure and writes values into flat vector.
pub fn json_to_flat_input(json: &JsonValue) -> Option<Vec<BigInt>> {
    let mut values = Vec::new();

    match json {
        JsonValue::Null => {
            log::warn!("Encountered 'null' in witness' json. Ignoring.");
        },
        JsonValue::Short(s) => {
            let value = bigint_from_str(s.as_str())?;
            values.push(value);
        },
        JsonValue::String(s) => {
            let value = bigint_from_str(s.as_str())?;
            values.push(value);
        },
        JsonValue::Number(_) => {
            log::error!("Number type is not supported for witness, use strings instead.");
            return None
        },
        JsonValue::Boolean(boolean) => {
            match *boolean {
                true => values.push(1.into()),
                false => values.push(0.into())
            }
        },
        JsonValue::Object(object) => {
            for (_key, value) in object.iter() {
                let mut nested_values = json_to_flat_input(value)?;
                values.append(&mut nested_values);
            }
        },
        JsonValue::Array(array) => {
            for value in array {
                let mut nested_values = json_to_flat_input(value)?;
                values.append(&mut nested_values);
            }
        },
    }

    Some(values)
}


#[cfg(test)]
mod tests {
    use crate::io::json_to_flat_input;
    use num_bigint::BigInt;

    static TEST_JSON: &str = r#"{
    "strings" : ["1", "2", "3"],
    "boolean" : [false, true],
    "nested": {
        "hex": ["0xdeadbeef", "0xDEADBEEF", "0xDeAdBeEf"],
        "num": ["42"]
    }
}"#;

    #[test]
    fn test() {
        let _ = env_logger::builder().is_test(true).try_init();

        let json = json::parse(TEST_JSON).unwrap();
        let witness = json_to_flat_input(&json).unwrap();
        let expected: Vec<BigInt> = vec![
            1.into(), 2.into(), 3.into(),
            0.into(), 1.into(),
            0xDEADBEEFu32.into(), 0xDEADBEEFu32.into(), 0xDEADBEEFu32.into(),
            42.into(),
        ];

        assert_eq!(witness, expected);
    }
}
