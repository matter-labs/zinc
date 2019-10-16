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
        let temp_000001 = r1cs::allocate_number(system.namespace(|| "temp_000001"), "1")?;
        let temp_000002 = r1cs::allocate_number(system.namespace(|| "temp_000002"), "2")?;
        let temp_000003 = r1cs::allocate_number(system.namespace(|| "temp_000003"), "3")?;
        let temp_000004 = r1cs::allocate_number(system.namespace(|| "temp_000004"), "4")?;
        let temp_000005 = [temp_000001, temp_000002, temp_000003, temp_000004];
        let temp_000006 = r1cs::allocate_number(system.namespace(|| "temp_000006"), "5")?;
        let temp_000007 = r1cs::allocate_number(system.namespace(|| "temp_000007"), "6")?;
        let temp_000008 = r1cs::allocate_number(system.namespace(|| "temp_000008"), "7")?;
        let temp_000009 = r1cs::allocate_number(system.namespace(|| "temp_000009"), "8")?;
        let temp_000010 = [temp_000006, temp_000007, temp_000008, temp_000009];
        let temp_000011 = r1cs::allocate_number(system.namespace(|| "temp_000011"), "9")?;
        let temp_000012 = r1cs::allocate_number(system.namespace(|| "temp_000012"), "10")?;
        let temp_000013 = r1cs::allocate_number(system.namespace(|| "temp_000013"), "11")?;
        let temp_000014 = r1cs::allocate_number(system.namespace(|| "temp_000014"), "12")?;
        let temp_000015 = [temp_000011, temp_000012, temp_000013, temp_000014];
        let temp_000016 = r1cs::allocate_number(system.namespace(|| "temp_000016"), "13")?;
        let temp_000017 = r1cs::allocate_number(system.namespace(|| "temp_000017"), "14")?;
        let temp_000018 = r1cs::allocate_number(system.namespace(|| "temp_000018"), "15")?;
        let temp_000019 = r1cs::allocate_number(system.namespace(|| "temp_000019"), "16")?;
        let temp_000020 = [temp_000016, temp_000017, temp_000018, temp_000019];
        let temp_000021 = [temp_000005, temp_000010, temp_000015, temp_000020];
        let mut array_double: [[AllocatedNum<Bn256>; 4]; 4] = temp_000021;
        let temp_000022 = r1cs::allocate_number(system.namespace(|| "temp_000022"), "42")?;
        ((array_double[1])[3]) = temp_000022;
        let temp_000023 = r1cs::allocate_number(system.namespace(|| "temp_000023"), "111")?;
        ((array_double[2])[2]) = temp_000023;
        let temp_000024 = r1cs::allocate_number(system.namespace(|| "temp_000024"), "255")?;
        ((array_double[3])[1]) = temp_000024;
        let temp_000025 = r1cs::allocate_number(system.namespace(|| "temp_000025"), "42")?;
        let temp_000026 = r1cs::equals_number(
            system.namespace(|| "temp_000026"),
            &((array_double[1])[3]),
            &temp_000025,
            254,
        )?;
        r1cs::require(system.namespace(|| "L18"), &temp_000026, "L18");
        let temp_000027 = r1cs::allocate_number(system.namespace(|| "temp_000027"), "111")?;
        let temp_000028 = r1cs::equals_number(
            system.namespace(|| "temp_000028"),
            &((array_double[2])[2]),
            &temp_000027,
            254,
        )?;
        r1cs::require(system.namespace(|| "L19"), &temp_000028, "L19");
        let temp_000029 = r1cs::allocate_number(system.namespace(|| "temp_000029"), "255")?;
        let temp_000030 = r1cs::equals_number(
            system.namespace(|| "temp_000030"),
            &((array_double[3])[1]),
            &temp_000029,
            254,
        )?;
        r1cs::require(system.namespace(|| "L20"), &temp_000030, "L20");
        Ok(())
    }
}
