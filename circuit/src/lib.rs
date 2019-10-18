#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(clippy::all)]

use r1cs::AllocatedNum;
use r1cs::Bn256;
use r1cs::Boolean;
use r1cs::Circuit;
use r1cs::ConstraintSystem;
use r1cs::Fr;
use r1cs::SynthesisError;

#[derive(Default)]
pub struct GeneratedCircuit {}

impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<S: ConstraintSystem<Bn256>>(self, system: &mut S) -> Result<(), SynthesisError> {
        let temp_000001 = r1cs::allocate_number(system.namespace(|| "temp_000001"), "2")?;
        let temp_000002 = r1cs::allocate_number(system.namespace(|| "temp_000002"), "1")?;
        let temp_000003 = r1cs::allocate_number(system.namespace(|| "temp_000003"), "0")?;
        let temp_000004 = r1cs::allocate_number(system.namespace(|| "temp_000004"), "3")?;
        let temp_000005 = [temp_000001, temp_000002, temp_000003, temp_000004];
        let original = temp_000005;
        let temp_000006 = r1cs::allocate_number(system.namespace(|| "temp_000006"), "0")?;
        let temp_000007 = r1cs::allocate_number(system.namespace(|| "temp_000007"), "0")?;
        let temp_000008 = r1cs::allocate_number(system.namespace(|| "temp_000008"), "0")?;
        let temp_000009 = r1cs::allocate_number(system.namespace(|| "temp_000009"), "0")?;
        let temp_000010 = [temp_000006, temp_000007, temp_000008, temp_000009];
        let mut array = temp_000010;
        let temp_000012 = {
            let temp_000013 = r1cs::allocate_number(system.namespace(|| "temp_000013"), "0")?;
            (array[0]) = temp_000013;
        };
        let temp_000014 = r1cs::allocate_number(system.namespace(|| "temp_000014"), "0")?;
        let temp_000015 = r1cs::equals_number(
            system.namespace(|| "temp_000015"),
            &temp_000014,
            &(original[0]),
            8,
        )?;
        let temp_000017 = {
            let temp_000018 = r1cs::allocate_number(system.namespace(|| "temp_000018"), "1")?;
            (array[1]) = temp_000018;
        };
        let temp_000019 = r1cs::allocate_number(system.namespace(|| "temp_000019"), "1")?;
        let temp_000020 = r1cs::equals_number(
            system.namespace(|| "temp_000020"),
            &temp_000019,
            &(original[1]),
            8,
        )?;
        let temp_000022 = {
            let temp_000023 = r1cs::allocate_number(system.namespace(|| "temp_000023"), "2")?;
            (array[2]) = temp_000023;
        };
        let temp_000024 = r1cs::allocate_number(system.namespace(|| "temp_000024"), "2")?;
        let temp_000025 = r1cs::equals_number(
            system.namespace(|| "temp_000025"),
            &temp_000024,
            &(original[2]),
            8,
        )?;
        let temp_000027 = {
            let temp_000028 = r1cs::allocate_number(system.namespace(|| "temp_000028"), "3")?;
            (array[3]) = temp_000028;
        };
        let temp_000029 = r1cs::allocate_number(system.namespace(|| "temp_000029"), "3")?;
        let temp_000030 = r1cs::equals_number(
            system.namespace(|| "temp_000030"),
            &temp_000029,
            &(original[3]),
            8,
        )?;
        let temp_000031 = r1cs::allocate_number(system.namespace(|| "temp_000031"), "0")?;
        let temp_000032 = r1cs::equals_number(
            system.namespace(|| "temp_000032"),
            &temp_000031,
            &(array[0]),
            8,
        )?;
        r1cs::require(system.namespace(|| "L23"), &temp_000032, "L23");
        let temp_000033 = r1cs::allocate_number(system.namespace(|| "temp_000033"), "1")?;
        let temp_000034 = r1cs::equals_number(
            system.namespace(|| "temp_000034"),
            &temp_000033,
            &(array[1]),
            8,
        )?;
        r1cs::require(system.namespace(|| "L24"), &temp_000034, "L24");
        let temp_000035 = r1cs::allocate_number(system.namespace(|| "temp_000035"), "2")?;
        let temp_000036 = r1cs::equals_number(
            system.namespace(|| "temp_000036"),
            &temp_000035,
            &(array[2]),
            8,
        )?;
        r1cs::require(system.namespace(|| "L25"), &temp_000036, "L25");
        let temp_000037 = r1cs::allocate_number(system.namespace(|| "temp_000037"), "3")?;
        let temp_000038 = r1cs::equals_number(
            system.namespace(|| "temp_000038"),
            &temp_000037,
            &(array[3]),
            8,
        )?;
        r1cs::require(system.namespace(|| "L26"), &temp_000038, "L26");
        let temp_000039 = r1cs::allocate_boolean(system.namespace(|| "temp_000039"), false)?;
        let temp_000040 = r1cs::allocate_boolean(system.namespace(|| "temp_000040"), true)?;
        let temp_000041 = r1cs::not_equals_boolean(
            system.namespace(|| "temp_000041"),
            &temp_000040,
            &temp_000039,
        )?;
        r1cs::require(system.namespace(|| "L28"), &temp_000041, "L28");
        Ok(())
    }
}
