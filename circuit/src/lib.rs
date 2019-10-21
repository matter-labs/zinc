#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(clippy::all)]

use r1cs::ConstraintSystem;
use r1cs::Circuit;
use r1cs::SynthesisError;
use r1cs::Bn256;
use r1cs::Fr;
use r1cs::Boolean;
use r1cs::AllocatedNum;

#[derive(Default)]
pub struct GeneratedCircuit {
    pub input: Fr,
    pub result: Fr,
}

impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<S: ConstraintSystem<Bn256>>(self, system: &mut S) -> Result<(), SynthesisError> {
        let input = r1cs::allocate_input(system.namespace(|| "input_input"), || Ok(self.input), 254)?.0;
        let result = r1cs::allocate_witness(system.namespace(|| "witness_result"), || Ok(self.result), 254)?.0;
        let temp_000001 = r1cs::allocate_number(system.namespace(|| "temp_000001"), "42")?;
        let var_1 = temp_000001;
        let temp_000002 = r1cs::allocate_number(system.namespace(|| "temp_000002"), "42")?;
        let mut var_mutable = temp_000002;
        let temp_000003 = r1cs::allocate_number(system.namespace(|| "temp_000003"), "43")?;
        var_mutable = temp_000003;
        dbg!(&var_mutable.get_value());
        let temp_000004 = r1cs::allocate_number(system.namespace(|| "temp_000004"), "43")?;
        let temp_000005 = r1cs::equals_number(system.namespace(|| "temp_000005"), &var_mutable, &temp_000004, 8)?;
        r1cs::require(system.namespace(|| "L67C1"), &temp_000005, "L67C1");
        let temp_000006 = r1cs::allocate_number(system.namespace(|| "temp_000006"), "42")?;
        let byte: AllocatedNum<Bn256> = temp_000006;
        let temp_000007 = r1cs::allocate_number(system.namespace(|| "temp_000007"), "42")?;
        let temp_000008 = r1cs::negate(system.namespace(|| "temp_000008"), &temp_000007, 254)?.0;
        let signed = temp_000008;
        let temp_000009 = r1cs::allocate_number(system.namespace(|| "temp_000009"), "18446744073709551615")?;
        let temp_000010 = r1cs::cast(system.namespace(|| "temp_000010"), &temp_000009, 254)?;
        let temp_000011 = r1cs::cast(system.namespace(|| "temp_000011"), &byte, 254)?;
        let temp_000012 = r1cs::add(system.namespace(|| "temp_000012"), &temp_000010, &temp_000011, 254)?.0;
        let big_integer: AllocatedNum<Bn256> = temp_000012;
        let temp_000013 = r1cs::allocate_number(system.namespace(|| "temp_000013"), "10000")?;
        let temp_000014 = r1cs::allocate_number(system.namespace(|| "temp_000014"), "54354")?;
        let temp_000015 = r1cs::add(system.namespace(|| "temp_000015"), &temp_000014, &temp_000013, 254)?.0;
        let temp_000016 = r1cs::allocate_number(system.namespace(|| "temp_000016"), "5422")?;
        let temp_000017 = r1cs::subtract(system.namespace(|| "temp_000017"), &temp_000015, &temp_000016, 254)?.0;
        let inferred = temp_000017;
        let temp_000018 = r1cs::allocate_number(system.namespace(|| "temp_000018"), "1")?;
        let temp_000019 = r1cs::allocate_number(system.namespace(|| "temp_000019"), "1")?;
        let temp_000020 = r1cs::add(system.namespace(|| "temp_000020"), &temp_000019, &temp_000018, 254)?.0;
        let temp_000021 = r1cs::allocate_number(system.namespace(|| "temp_000021"), "7")?;
        let temp_000022 = r1cs::subtract(system.namespace(|| "temp_000022"), &temp_000021, &temp_000020, 254)?.0;
        let temp_000023 = r1cs::allocate_number(system.namespace(|| "temp_000023"), "15")?;
        let temp_000024 = r1cs::add(system.namespace(|| "temp_000024"), &temp_000023, &temp_000022, 254)?.0;
        let temp_000025 = r1cs::allocate_number(system.namespace(|| "temp_000025"), "3")?;
        let temp_000026 = r1cs::multiply(system.namespace(|| "temp_000026"), &temp_000024, &temp_000025, 254)?.0;
        let temp_000027 = r1cs::allocate_number(system.namespace(|| "temp_000027"), "1")?;
        let temp_000028 = r1cs::allocate_number(system.namespace(|| "temp_000028"), "1")?;
        let temp_000029 = r1cs::add(system.namespace(|| "temp_000029"), &temp_000028, &temp_000027, 254)?.0;
        let temp_000030 = r1cs::allocate_number(system.namespace(|| "temp_000030"), "7")?;
        let temp_000031 = r1cs::subtract(system.namespace(|| "temp_000031"), &temp_000030, &temp_000029, 254)?.0;
        let temp_000032 = r1cs::allocate_number(system.namespace(|| "temp_000032"), "15")?;
        let temp_000033 = r1cs::add(system.namespace(|| "temp_000033"), &temp_000032, &temp_000031, 254)?.0;
        let temp_000034 = r1cs::allocate_number(system.namespace(|| "temp_000034"), "4")?;
        let temp_000035 = r1cs::multiply(system.namespace(|| "temp_000035"), &temp_000033, &temp_000034, 254)?.0;
        let temp_000036 = r1cs::lesser(system.namespace(|| "temp_000036"), &temp_000026, &temp_000035, 254)?;
        let expression = temp_000036;
        let temp_000037 = {
            let temp_000038 = {
                let temp_000039 = {
                    let temp_000040 = r1cs::allocate_number(system.namespace(|| "temp_000040"), "4")?;
                    temp_000040
                };
                let temp_000041 = r1cs::allocate_number(system.namespace(|| "temp_000041"), "3")?;
                let temp_000042 = r1cs::add(system.namespace(|| "temp_000042"), &temp_000041, &temp_000039, 254)?.0;
                let temp_000043 = r1cs::allocate_number(system.namespace(|| "temp_000043"), "3")?;
                let temp_000044 = r1cs::add(system.namespace(|| "temp_000044"), &temp_000042, &temp_000043, 254)?.0;
                temp_000044
            };
            let temp_000045 = r1cs::allocate_number(system.namespace(|| "temp_000045"), "2")?;
            let temp_000046 = r1cs::add(system.namespace(|| "temp_000046"), &temp_000045, &temp_000038, 254)?.0;
            let temp_000047 = r1cs::allocate_number(system.namespace(|| "temp_000047"), "2")?;
            let temp_000048 = r1cs::add(system.namespace(|| "temp_000048"), &temp_000046, &temp_000047, 254)?.0;
            temp_000048
        };
        let temp_000049 = r1cs::allocate_number(system.namespace(|| "temp_000049"), "1")?;
        let temp_000050 = r1cs::add(system.namespace(|| "temp_000050"), &temp_000049, &temp_000037, 254)?.0;
        let temp_000051 = r1cs::allocate_number(system.namespace(|| "temp_000051"), "1")?;
        let temp_000052 = r1cs::add(system.namespace(|| "temp_000052"), &temp_000050, &temp_000051, 254)?.0;
        let pyramid = temp_000052;
        let temp_000053 = r1cs::allocate_number(system.namespace(|| "temp_000053"), "16")?;
        let temp_000054 = r1cs::equals_number(system.namespace(|| "temp_000054"), &pyramid, &temp_000053, 8)?;
        r1cs::require(system.namespace(|| "L121C1"), &temp_000054, "L121C1");
        struct Test { x: AllocatedNum<Bn256>, y: AllocatedNum<Bn256>, z: AllocatedNum<Bn256> }
        let temp_000055 = r1cs::allocate_number(system.namespace(|| "temp_000055"), "1")?;
        let temp_000056 = r1cs::allocate_number(system.namespace(|| "temp_000056"), "2")?;
        let temp_000057 = r1cs::allocate_number(system.namespace(|| "temp_000057"), "3")?;
        let temp_000058 = Test { x: temp_000055, y: temp_000056, z: temp_000057 };
        let mut test = temp_000058;
        let temp_000059 = r1cs::allocate_number(system.namespace(|| "temp_000059"), "5")?;
        (test.x) = temp_000059;
        let temp_000060 = r1cs::allocate_number(system.namespace(|| "temp_000060"), "7")?;
        (test.y) = temp_000060;
        let temp_000061 = r1cs::allocate_number(system.namespace(|| "temp_000061"), "9")?;
        (test.z) = temp_000061;
        let temp_000062 = r1cs::allocate_number(system.namespace(|| "temp_000062"), "5")?;
        let temp_000063 = r1cs::equals_number(system.namespace(|| "temp_000063"), &(test.x), &temp_000062, 8)?;
        r1cs::require(system.namespace(|| "L142C1"), &temp_000063, "L142C1");
        let temp_000064 = r1cs::allocate_number(system.namespace(|| "temp_000064"), "7")?;
        let temp_000065 = r1cs::equals_number(system.namespace(|| "temp_000065"), &(test.y), &temp_000064, 8)?;
        r1cs::require(system.namespace(|| "L143C1"), &temp_000065, "L143C1");
        let temp_000066 = r1cs::allocate_number(system.namespace(|| "temp_000066"), "9")?;
        let temp_000067 = r1cs::equals_number(system.namespace(|| "temp_000067"), &(test.z), &temp_000066, 8)?;
        r1cs::require(system.namespace(|| "L144C1"), &temp_000067, "L144C1");
        let temp_000068 = r1cs::allocate_number(system.namespace(|| "temp_000068"), "1")?;
        let temp_000069 = r1cs::allocate_number(system.namespace(|| "temp_000069"), "2")?;
        let temp_000070 = r1cs::allocate_number(system.namespace(|| "temp_000070"), "3")?;
        let temp_000071 = r1cs::allocate_number(system.namespace(|| "temp_000071"), "4")?;
        let temp_000072 = [temp_000068, temp_000069, temp_000070, temp_000071];
        let temp_000073 = r1cs::allocate_number(system.namespace(|| "temp_000073"), "5")?;
        let temp_000074 = r1cs::allocate_number(system.namespace(|| "temp_000074"), "6")?;
        let temp_000075 = r1cs::allocate_number(system.namespace(|| "temp_000075"), "7")?;
        let temp_000076 = r1cs::allocate_number(system.namespace(|| "temp_000076"), "8")?;
        let temp_000077 = [temp_000073, temp_000074, temp_000075, temp_000076];
        let temp_000078 = r1cs::allocate_number(system.namespace(|| "temp_000078"), "9")?;
        let temp_000079 = r1cs::allocate_number(system.namespace(|| "temp_000079"), "10")?;
        let temp_000080 = r1cs::allocate_number(system.namespace(|| "temp_000080"), "11")?;
        let temp_000081 = r1cs::allocate_number(system.namespace(|| "temp_000081"), "12")?;
        let temp_000082 = [temp_000078, temp_000079, temp_000080, temp_000081];
        let temp_000083 = r1cs::allocate_number(system.namespace(|| "temp_000083"), "13")?;
        let temp_000084 = r1cs::allocate_number(system.namespace(|| "temp_000084"), "14")?;
        let temp_000085 = r1cs::allocate_number(system.namespace(|| "temp_000085"), "15")?;
        let temp_000086 = r1cs::allocate_number(system.namespace(|| "temp_000086"), "16")?;
        let temp_000087 = [temp_000083, temp_000084, temp_000085, temp_000086];
        let temp_000088 = [temp_000072, temp_000077, temp_000082, temp_000087];
        let mut array_double: [[AllocatedNum<Bn256>; 4]; 4] = temp_000088;
        let temp_000089 = r1cs::allocate_number(system.namespace(|| "temp_000089"), "42")?;
        ((array_double[1])[3]) = temp_000089;
        let temp_000090 = r1cs::allocate_number(system.namespace(|| "temp_000090"), "111")?;
        ((array_double[2])[2]) = temp_000090;
        let temp_000091 = r1cs::allocate_number(system.namespace(|| "temp_000091"), "255")?;
        ((array_double[3])[1]) = temp_000091;
        let temp_000092 = r1cs::allocate_number(system.namespace(|| "temp_000092"), "42")?;
        let temp_000093 = r1cs::equals_number(system.namespace(|| "temp_000093"), &((array_double[1])[3]), &temp_000092, 8)?;
        r1cs::require(system.namespace(|| "L161C1"), &temp_000093, "L161C1");
        let temp_000094 = r1cs::allocate_number(system.namespace(|| "temp_000094"), "111")?;
        let temp_000095 = r1cs::equals_number(system.namespace(|| "temp_000095"), &((array_double[2])[2]), &temp_000094, 8)?;
        r1cs::require(system.namespace(|| "L162C1"), &temp_000095, "L162C1");
        let temp_000096 = r1cs::allocate_number(system.namespace(|| "temp_000096"), "255")?;
        let temp_000097 = r1cs::equals_number(system.namespace(|| "temp_000097"), &((array_double[3])[1]), &temp_000096, 8)?;
        r1cs::require(system.namespace(|| "L163C1"), &temp_000097, "L163C1");
        let temp_000098 = r1cs::allocate_number(system.namespace(|| "temp_000098"), "0")?;
        let mut sum: AllocatedNum<Bn256> = temp_000098;
        for i_index in 0..=5 {
            let i = r1cs::allocate_number(system.namespace(|| format!("temp_000098_{}", i_index)), i_index.to_string().as_str())?;
            let temp_000099 = r1cs::add(system.namespace(|| format!("temp_000099_{}", i_index)), &sum, &i, 254)?.0;
            sum = temp_000099;
            for j_index in 0..=5 {
                let j = r1cs::allocate_number(system.namespace(|| format!("temp_000099_{}_{}", i_index, j_index)), j_index.to_string().as_str())?;
                let temp_000100 = r1cs::add(system.namespace(|| format!("temp_000100_{}_{}", i_index, j_index)), &sum, &j, 254)?.0;
                sum = temp_000100;
            }
        }
        let temp_000101 = r1cs::allocate_number(system.namespace(|| "temp_000101"), "105")?;
        let temp_000102 = r1cs::cast(system.namespace(|| "temp_000102"), &temp_000101, 254)?;
        let temp_000103 = r1cs::equals_number(system.namespace(|| "temp_000103"), &sum, &temp_000102, 64)?;
        r1cs::require(system.namespace(|| "L178C1"), &temp_000103, "L178C1");
        let temp_000104 = r1cs::allocate_number(system.namespace(|| "temp_000104"), "0")?;
        let mut s: AllocatedNum<Bn256> = temp_000104;
        for k_index in 0..20 {
            let k = r1cs::allocate_number(system.namespace(|| format!("temp_000104_{}", k_index)), k_index.to_string().as_str())?;
            let temp_000105 = r1cs::allocate_number(system.namespace(|| format!("temp_000105_{}", k_index)), "10")?;
            let temp_000106 = r1cs::cast(system.namespace(|| format!("temp_000106_{}", k_index)), &temp_000105, 254)?;
            let temp_000107 = r1cs::lesser(system.namespace(|| format!("temp_000107_{}", k_index)), &k, &temp_000106, 254)?;
            if temp_000107.get_value().expect("Always returns a value") {
                let temp_000108 = r1cs::add(system.namespace(|| format!("temp_000108_{}", k_index)), &s, &k, 254)?.0;
                s = temp_000108;
            } else { break; }
        }
        let temp_000109 = r1cs::allocate_number(system.namespace(|| "temp_000109"), "55")?;
        let temp_000110 = r1cs::cast(system.namespace(|| "temp_000110"), &temp_000109, 254)?;
        let temp_000111 = r1cs::equals_number(system.namespace(|| "temp_000111"), &s, &temp_000110, 64)?;
        r1cs::require(system.namespace(|| "L196C1"), &temp_000111, "L196C1");
        let temp_000113 = {
            let temp_000114 = r1cs::allocate_number(system.namespace(|| "temp_000114"), "1")?;
            temp_000114
        };
        let temp_000116 = {
            let temp_000118 = {
                let temp_000119 = r1cs::allocate_number(system.namespace(|| "temp_000119"), "42")?;
                temp_000119
            };
            let temp_000120 = {
                let temp_000121 = r1cs::allocate_number(system.namespace(|| "temp_000121"), "2")?;
                temp_000121
            };
            let temp_000122 = r1cs::allocate_boolean(system.namespace(|| "temp_000122"), true)?;
            let temp_000117 = r1cs::conditional(system.namespace(|| "temp_000117"), &temp_000118, &temp_000120, &temp_000122)?;
            temp_000117
        };
        let temp_000123 = {
            let temp_000124 = r1cs::allocate_number(system.namespace(|| "temp_000124"), "3")?;
            temp_000124
        };
        let temp_000125 = r1cs::allocate_boolean(system.namespace(|| "temp_000125"), true)?;
        let temp_000115 = r1cs::conditional(system.namespace(|| "temp_000115"), &temp_000116, &temp_000123, &temp_000125)?;
        let temp_000126 = r1cs::allocate_boolean(system.namespace(|| "temp_000126"), false)?;
        let temp_000112 = r1cs::conditional(system.namespace(|| "temp_000112"), &temp_000113, &temp_000115, &temp_000126)?;
        let branch = temp_000112;
        let temp_000127 = r1cs::allocate_number(system.namespace(|| "temp_000127"), "42")?;
        let temp_000128 = r1cs::equals_number(system.namespace(|| "temp_000128"), &branch, &temp_000127, 8)?;
        r1cs::require(system.namespace(|| "L214C1"), &temp_000128, "L214C1");
        let temp_000129 = r1cs::allocate_number(system.namespace(|| "temp_000129"), "0")?;
        let mut value_1: AllocatedNum<Bn256> = temp_000129;
        let temp_000130 = r1cs::allocate_number(system.namespace(|| "temp_000130"), "1")?;
        let mut value_2: AllocatedNum<Bn256> = temp_000130;
        let mut fibo = value_1.clone();
        for l_index in 1..=6 {
            let l = r1cs::allocate_number(system.namespace(|| format!("temp_000130_{}", l_index)), l_index.to_string().as_str())?;
            let temp_000131 = r1cs::add(system.namespace(|| format!("temp_000131_{}", l_index)), &value_1, &value_2, 254)?.0;
            fibo = temp_000131;
            value_1 = value_2.clone();
            value_2 = fibo.clone();
        }
        let temp_000132 = r1cs::allocate_number(system.namespace(|| "temp_000132"), "13")?;
        let temp_000133 = r1cs::cast(system.namespace(|| "temp_000133"), &temp_000132, 254)?;
        let temp_000134 = r1cs::equals_number(system.namespace(|| "temp_000134"), &fibo, &temp_000133, 254)?;
        r1cs::require(system.namespace(|| "L232C1"), &temp_000134, "L232C1");
        let temp_000135 = r1cs::allocate_number(system.namespace(|| "temp_000135"), "1")?;
        let mut fact: AllocatedNum<Bn256> = temp_000135;
        for m_index in 2..6 {
            let m = r1cs::allocate_number(system.namespace(|| format!("temp_000135_{}", m_index)), m_index.to_string().as_str())?;
            let temp_000136 = r1cs::cast(system.namespace(|| format!("temp_000136_{}", m_index)), &m, 254)?;
            let temp_000137 = r1cs::multiply(system.namespace(|| format!("temp_000137_{}", m_index)), &fact, &temp_000136, 254)?.0;
            fact = temp_000137;
        }
        let temp_000138 = r1cs::allocate_number(system.namespace(|| "temp_000138"), "120")?;
        let temp_000139 = r1cs::cast(system.namespace(|| "temp_000139"), &temp_000138, 254)?;
        let temp_000140 = r1cs::equals_number(system.namespace(|| "temp_000140"), &fact, &temp_000139, 254)?;
        r1cs::require(system.namespace(|| "L244C1"), &temp_000140, "L244C1");
        struct MyStruct { data: AllocatedNum<Bn256> }
        let temp_000141 = r1cs::allocate_number(system.namespace(|| "temp_000141"), "42")?;
        let temp_000142 = MyStruct { data: temp_000141 };
        let mut payload = temp_000142;
        let temp_000143 = r1cs::allocate_number(system.namespace(|| "temp_000143"), "1")?;
        let temp_000144 = r1cs::allocate_number(system.namespace(|| "temp_000144"), "2")?;
        let temp_000145 = r1cs::allocate_number(system.namespace(|| "temp_000145"), "3")?;
        let temp_000146 = [temp_000143, temp_000144, temp_000145];
        let temp_000147 = r1cs::allocate_number(system.namespace(|| "temp_000147"), "4")?;
        let temp_000148 = r1cs::allocate_number(system.namespace(|| "temp_000148"), "5")?;
        let temp_000149 = r1cs::allocate_number(system.namespace(|| "temp_000149"), "6")?;
        let temp_000150 = [temp_000147, temp_000148, temp_000149];
        let temp_000151 = [temp_000146, temp_000150];
        let mega_array = temp_000151;
        let temp_000153 = {
            let temp_000154 = r1cs::allocate_number(system.namespace(|| "temp_000154"), "1")?;
            let temp_000155 = r1cs::allocate_number(system.namespace(|| "temp_000155"), "1")?;
            let temp_000156 = r1cs::add(system.namespace(|| "temp_000156"), &temp_000155, &temp_000154, 254)?.0;
            let temp_000157 = r1cs::allocate_number(system.namespace(|| "temp_000157"), "7")?;
            let temp_000158 = r1cs::subtract(system.namespace(|| "temp_000158"), &temp_000157, &temp_000156, 254)?.0;
            let temp_000159 = r1cs::allocate_number(system.namespace(|| "temp_000159"), "15")?;
            let temp_000160 = r1cs::add(system.namespace(|| "temp_000160"), &temp_000159, &temp_000158, 254)?.0;
            let temp_000161 = r1cs::allocate_number(system.namespace(|| "temp_000161"), "3")?;
            let temp_000162 = r1cs::multiply(system.namespace(|| "temp_000162"), &temp_000160, &temp_000161, 254)?.0;
            let temp_000163 = r1cs::add(system.namespace(|| "temp_000163"), &(payload.data), &((mega_array[1])[1]), 254)?.0;
            let temp_000164 = r1cs::allocate_number(system.namespace(|| "temp_000164"), "2")?;
            let temp_000165 = r1cs::add(system.namespace(|| "temp_000165"), &temp_000164, &temp_000163, 254)?.0;
            let temp_000166 = r1cs::subtract(system.namespace(|| "temp_000166"), &temp_000162, &temp_000165, 254)?.0;
            let r = temp_000166;
            let temp_000167 = r1cs::allocate_number(system.namespace(|| "temp_000167"), "11")?;
            let temp_000168 = r1cs::equals_number(system.namespace(|| "temp_000168"), &r, &temp_000167, 8)?;
            r1cs::require(system.namespace(|| "L263C5"), &temp_000168, "L263C5");
        };
        let temp_000169 = {
            let temp_000170 = r1cs::allocate_number(system.namespace(|| "temp_000170"), "50")?;
            (payload.data) = temp_000170;
            let temp_000171 = r1cs::allocate_number(system.namespace(|| "temp_000171"), "5")?;
            let temp_000172 = r1cs::add(system.namespace(|| "temp_000172"), &(payload.data), &temp_000171, 254)?.0;
            let temp_000173 = r1cs::allocate_number(system.namespace(|| "temp_000173"), "55")?;
            let temp_000174 = r1cs::equals_number(system.namespace(|| "temp_000174"), &temp_000172, &temp_000173, 8)?;
            r1cs::require(system.namespace(|| "L266C5"), &temp_000174, "L266C5");
        };
        let temp_000175 = r1cs::allocate_number(system.namespace(|| "temp_000175"), "1")?;
        let temp_000176 = r1cs::equals_number(system.namespace(|| "temp_000176"), &((mega_array[0])[0]), &temp_000175, 8)?;
        Ok(())
    }
}
