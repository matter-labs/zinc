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
        let temp_000002 = r1cs::allocate_number(system.namespace(|| "temp_000002"), "41")?;
        let mut var_mutable = temp_000002;
        let temp_000003 = r1cs::allocate_number(system.namespace(|| "temp_000003"), "42")?;
        var_mutable = temp_000003;
        dbg!(&var_mutable.get_value());
        let temp_000004 = r1cs::allocate_number(system.namespace(|| "temp_000004"), "42")?;
        let temp_000005 = r1cs::equals_number(system.namespace(|| "temp_000005"), &var_mutable, &temp_000004, 8)?;
        r1cs::require(system.namespace(|| "L69C1"), &temp_000005, "L69C1");
        let temp_000006 = r1cs::allocate_number(system.namespace(|| "temp_000006"), "42")?;
        let temp_000007 = r1cs::equals_number(system.namespace(|| "temp_000007"), &var_mutable, &temp_000006, 8)?;
        r1cs::require(system.namespace(|| "expects the Answer to the Ultimate Question of Life, the Universe, and Everything"), &temp_000007, "expects the Answer to the Ultimate Question of Life, the Universe, and Everything");
        let temp_000008 = r1cs::allocate_number(system.namespace(|| "temp_000008"), "42")?;
        let byte: AllocatedNum<Bn256> = temp_000008;
        let temp_000009 = r1cs::allocate_number(system.namespace(|| "temp_000009"), "42")?;
        let temp_000010 = r1cs::negate(system.namespace(|| "temp_000010"), &temp_000009, 254)?.0;
        let signed = temp_000010;
        let temp_000011 = r1cs::allocate_number(system.namespace(|| "temp_000011"), "18446744073709551615")?;
        let temp_000012 = r1cs::cast(system.namespace(|| "temp_000012"), &temp_000011, 254)?;
        let temp_000013 = r1cs::cast(system.namespace(|| "temp_000013"), &byte, 254)?;
        let temp_000014 = r1cs::add(system.namespace(|| "temp_000014"), &temp_000012, &temp_000013, 254)?.0;
        let big_integer: AllocatedNum<Bn256> = temp_000014;
        let temp_000015 = r1cs::allocate_number(system.namespace(|| "temp_000015"), "10000")?;
        let temp_000016 = r1cs::allocate_number(system.namespace(|| "temp_000016"), "54354")?;
        let temp_000017 = r1cs::add(system.namespace(|| "temp_000017"), &temp_000016, &temp_000015, 254)?.0;
        let temp_000018 = r1cs::allocate_number(system.namespace(|| "temp_000018"), "5422")?;
        let temp_000019 = r1cs::subtract(system.namespace(|| "temp_000019"), &temp_000017, &temp_000018, 254)?.0;
        let inferred = temp_000019;
        let temp_000020 = r1cs::allocate_number(system.namespace(|| "temp_000020"), "1")?;
        let temp_000021 = r1cs::allocate_number(system.namespace(|| "temp_000021"), "1")?;
        let temp_000022 = r1cs::add(system.namespace(|| "temp_000022"), &temp_000021, &temp_000020, 254)?.0;
        let temp_000023 = r1cs::allocate_number(system.namespace(|| "temp_000023"), "7")?;
        let temp_000024 = r1cs::subtract(system.namespace(|| "temp_000024"), &temp_000023, &temp_000022, 254)?.0;
        let temp_000025 = r1cs::allocate_number(system.namespace(|| "temp_000025"), "15")?;
        let temp_000026 = r1cs::add(system.namespace(|| "temp_000026"), &temp_000025, &temp_000024, 254)?.0;
        let temp_000027 = r1cs::allocate_number(system.namespace(|| "temp_000027"), "3")?;
        let temp_000028 = r1cs::multiply(system.namespace(|| "temp_000028"), &temp_000026, &temp_000027, 254)?.0;
        let temp_000029 = r1cs::allocate_number(system.namespace(|| "temp_000029"), "1")?;
        let temp_000030 = r1cs::allocate_number(system.namespace(|| "temp_000030"), "1")?;
        let temp_000031 = r1cs::add(system.namespace(|| "temp_000031"), &temp_000030, &temp_000029, 254)?.0;
        let temp_000032 = r1cs::allocate_number(system.namespace(|| "temp_000032"), "7")?;
        let temp_000033 = r1cs::subtract(system.namespace(|| "temp_000033"), &temp_000032, &temp_000031, 254)?.0;
        let temp_000034 = r1cs::allocate_number(system.namespace(|| "temp_000034"), "15")?;
        let temp_000035 = r1cs::add(system.namespace(|| "temp_000035"), &temp_000034, &temp_000033, 254)?.0;
        let temp_000036 = r1cs::allocate_number(system.namespace(|| "temp_000036"), "4")?;
        let temp_000037 = r1cs::multiply(system.namespace(|| "temp_000037"), &temp_000035, &temp_000036, 254)?.0;
        let temp_000038 = r1cs::lesser(system.namespace(|| "temp_000038"), &temp_000028, &temp_000037, 254)?;
        let expression = temp_000038;
        let temp_000039 = {
            let temp_000040 = {
                let temp_000041 = {
                    let temp_000042 = r1cs::allocate_number(system.namespace(|| "temp_000042"), "4")?;
                    temp_000042
                };
                let temp_000043 = r1cs::allocate_number(system.namespace(|| "temp_000043"), "3")?;
                let temp_000044 = r1cs::add(system.namespace(|| "temp_000044"), &temp_000043, &temp_000041, 254)?.0;
                let temp_000045 = r1cs::allocate_number(system.namespace(|| "temp_000045"), "3")?;
                let temp_000046 = r1cs::add(system.namespace(|| "temp_000046"), &temp_000044, &temp_000045, 254)?.0;
                temp_000046
            };
            let temp_000047 = r1cs::allocate_number(system.namespace(|| "temp_000047"), "2")?;
            let temp_000048 = r1cs::add(system.namespace(|| "temp_000048"), &temp_000047, &temp_000040, 254)?.0;
            let temp_000049 = r1cs::allocate_number(system.namespace(|| "temp_000049"), "2")?;
            let temp_000050 = r1cs::add(system.namespace(|| "temp_000050"), &temp_000048, &temp_000049, 254)?.0;
            temp_000050
        };
        let temp_000051 = r1cs::allocate_number(system.namespace(|| "temp_000051"), "1")?;
        let temp_000052 = r1cs::add(system.namespace(|| "temp_000052"), &temp_000051, &temp_000039, 254)?.0;
        let temp_000053 = r1cs::allocate_number(system.namespace(|| "temp_000053"), "1")?;
        let temp_000054 = r1cs::add(system.namespace(|| "temp_000054"), &temp_000052, &temp_000053, 254)?.0;
        let pyramid = temp_000054;
        let temp_000055 = r1cs::allocate_number(system.namespace(|| "temp_000055"), "16")?;
        let temp_000056 = r1cs::equals_number(system.namespace(|| "temp_000056"), &pyramid, &temp_000055, 8)?;
        r1cs::require(system.namespace(|| "L124C1"), &temp_000056, "L124C1");
        struct Test { x: AllocatedNum<Bn256>, y: AllocatedNum<Bn256>, z: AllocatedNum<Bn256> }
        let temp_000057 = r1cs::allocate_number(system.namespace(|| "temp_000057"), "1")?;
        let temp_000058 = r1cs::allocate_number(system.namespace(|| "temp_000058"), "2")?;
        let temp_000059 = r1cs::allocate_number(system.namespace(|| "temp_000059"), "3")?;
        let temp_000060 = Test { x: temp_000057, y: temp_000058, z: temp_000059 };
        let mut test = temp_000060;
        let temp_000061 = r1cs::allocate_number(system.namespace(|| "temp_000061"), "5")?;
        (test.x) = temp_000061;
        let temp_000062 = r1cs::allocate_number(system.namespace(|| "temp_000062"), "7")?;
        (test.y) = temp_000062;
        let temp_000063 = r1cs::allocate_number(system.namespace(|| "temp_000063"), "9")?;
        (test.z) = temp_000063;
        let temp_000064 = r1cs::allocate_number(system.namespace(|| "temp_000064"), "5")?;
        let temp_000065 = r1cs::equals_number(system.namespace(|| "temp_000065"), &(test.x), &temp_000064, 8)?;
        r1cs::require(system.namespace(|| "L145C1"), &temp_000065, "L145C1");
        let temp_000066 = r1cs::allocate_number(system.namespace(|| "temp_000066"), "7")?;
        let temp_000067 = r1cs::equals_number(system.namespace(|| "temp_000067"), &(test.y), &temp_000066, 8)?;
        r1cs::require(system.namespace(|| "L146C1"), &temp_000067, "L146C1");
        let temp_000068 = r1cs::allocate_number(system.namespace(|| "temp_000068"), "9")?;
        let temp_000069 = r1cs::equals_number(system.namespace(|| "temp_000069"), &(test.z), &temp_000068, 8)?;
        r1cs::require(system.namespace(|| "L147C1"), &temp_000069, "L147C1");
        let temp_000070 = r1cs::allocate_number(system.namespace(|| "temp_000070"), "1")?;
        let temp_000071 = r1cs::allocate_number(system.namespace(|| "temp_000071"), "2")?;
        let temp_000072 = r1cs::allocate_number(system.namespace(|| "temp_000072"), "3")?;
        let temp_000073 = r1cs::allocate_number(system.namespace(|| "temp_000073"), "4")?;
        let temp_000074 = [temp_000070, temp_000071, temp_000072, temp_000073];
        let temp_000075 = r1cs::allocate_number(system.namespace(|| "temp_000075"), "5")?;
        let temp_000076 = r1cs::allocate_number(system.namespace(|| "temp_000076"), "6")?;
        let temp_000077 = r1cs::allocate_number(system.namespace(|| "temp_000077"), "7")?;
        let temp_000078 = r1cs::allocate_number(system.namespace(|| "temp_000078"), "8")?;
        let temp_000079 = [temp_000075, temp_000076, temp_000077, temp_000078];
        let temp_000080 = r1cs::allocate_number(system.namespace(|| "temp_000080"), "9")?;
        let temp_000081 = r1cs::allocate_number(system.namespace(|| "temp_000081"), "10")?;
        let temp_000082 = r1cs::allocate_number(system.namespace(|| "temp_000082"), "11")?;
        let temp_000083 = r1cs::allocate_number(system.namespace(|| "temp_000083"), "12")?;
        let temp_000084 = [temp_000080, temp_000081, temp_000082, temp_000083];
        let temp_000085 = r1cs::allocate_number(system.namespace(|| "temp_000085"), "13")?;
        let temp_000086 = r1cs::allocate_number(system.namespace(|| "temp_000086"), "14")?;
        let temp_000087 = r1cs::allocate_number(system.namespace(|| "temp_000087"), "15")?;
        let temp_000088 = r1cs::allocate_number(system.namespace(|| "temp_000088"), "16")?;
        let temp_000089 = [temp_000085, temp_000086, temp_000087, temp_000088];
        let temp_000090 = [temp_000074, temp_000079, temp_000084, temp_000089];
        let mut array_double: [[AllocatedNum<Bn256>; 4]; 4] = temp_000090;
        let temp_000091 = r1cs::allocate_number(system.namespace(|| "temp_000091"), "42")?;
        ((array_double[1])[3]) = temp_000091;
        let temp_000092 = r1cs::allocate_number(system.namespace(|| "temp_000092"), "111")?;
        ((array_double[2])[2]) = temp_000092;
        let temp_000093 = r1cs::allocate_number(system.namespace(|| "temp_000093"), "255")?;
        ((array_double[3])[1]) = temp_000093;
        let temp_000094 = r1cs::allocate_number(system.namespace(|| "temp_000094"), "42")?;
        let temp_000095 = r1cs::equals_number(system.namespace(|| "temp_000095"), &((array_double[1])[3]), &temp_000094, 8)?;
        r1cs::require(system.namespace(|| "L164C1"), &temp_000095, "L164C1");
        let temp_000096 = r1cs::allocate_number(system.namespace(|| "temp_000096"), "111")?;
        let temp_000097 = r1cs::equals_number(system.namespace(|| "temp_000097"), &((array_double[2])[2]), &temp_000096, 8)?;
        r1cs::require(system.namespace(|| "L165C1"), &temp_000097, "L165C1");
        let temp_000098 = r1cs::allocate_number(system.namespace(|| "temp_000098"), "255")?;
        let temp_000099 = r1cs::equals_number(system.namespace(|| "temp_000099"), &((array_double[3])[1]), &temp_000098, 8)?;
        r1cs::require(system.namespace(|| "L166C1"), &temp_000099, "L166C1");
        let temp_000100 = r1cs::allocate_number(system.namespace(|| "temp_000100"), "0")?;
        let mut sum: AllocatedNum<Bn256> = temp_000100;
        for i_index in 0..=5 {
            let i = r1cs::allocate_number(system.namespace(|| format!("temp_000100_{}", i_index)), i_index.to_string().as_str())?;
            let temp_000101 = r1cs::add(system.namespace(|| format!("temp_000101_{}", i_index)), &sum, &i, 254)?.0;
            sum = temp_000101;
            for j_index in 0..=5 {
                let j = r1cs::allocate_number(system.namespace(|| format!("temp_000101_{}_{}", i_index, j_index)), j_index.to_string().as_str())?;
                let temp_000102 = r1cs::add(system.namespace(|| format!("temp_000102_{}_{}", i_index, j_index)), &sum, &j, 254)?.0;
                sum = temp_000102;
            }
        }
        let temp_000103 = r1cs::allocate_number(system.namespace(|| "temp_000103"), "105")?;
        let temp_000104 = r1cs::cast(system.namespace(|| "temp_000104"), &temp_000103, 254)?;
        let temp_000105 = r1cs::equals_number(system.namespace(|| "temp_000105"), &sum, &temp_000104, 64)?;
        r1cs::require(system.namespace(|| "line_178_error_msg"), &temp_000105, "line_178_error_msg");
        let temp_000106 = r1cs::allocate_number(system.namespace(|| "temp_000106"), "0")?;
        let mut s: AllocatedNum<Bn256> = temp_000106;
        for k_index in 0..20 {
            let k = r1cs::allocate_number(system.namespace(|| format!("temp_000106_{}", k_index)), k_index.to_string().as_str())?;
            let temp_000107 = r1cs::allocate_number(system.namespace(|| format!("temp_000107_{}", k_index)), "10")?;
            let temp_000108 = r1cs::cast(system.namespace(|| format!("temp_000108_{}", k_index)), &temp_000107, 254)?;
            let temp_000109 = r1cs::lesser(system.namespace(|| format!("temp_000109_{}", k_index)), &k, &temp_000108, 254)?;
            if temp_000109.get_value().expect("Always returns a value") {
                let temp_000110 = r1cs::add(system.namespace(|| format!("temp_000110_{}", k_index)), &s, &k, 254)?.0;
                s = temp_000110;
            } else { break; }
        }
        let temp_000111 = r1cs::allocate_number(system.namespace(|| "temp_000111"), "55")?;
        let temp_000112 = r1cs::cast(system.namespace(|| "temp_000112"), &temp_000111, 254)?;
        let temp_000113 = r1cs::equals_number(system.namespace(|| "temp_000113"), &s, &temp_000112, 64)?;
        r1cs::require(system.namespace(|| "L199C1"), &temp_000113, "L199C1");
        let temp_000115 = {
            let temp_000116 = r1cs::allocate_number(system.namespace(|| "temp_000116"), "1")?;
            temp_000116
        };
        let temp_000118 = {
            let temp_000120 = {
                let temp_000121 = r1cs::allocate_number(system.namespace(|| "temp_000121"), "42")?;
                temp_000121
            };
            let temp_000122 = {
                let temp_000123 = r1cs::allocate_number(system.namespace(|| "temp_000123"), "2")?;
                temp_000123
            };
            let temp_000124 = r1cs::allocate_boolean(system.namespace(|| "temp_000124"), true)?;
            let temp_000119 = r1cs::conditional(system.namespace(|| "temp_000119"), &temp_000120, &temp_000122, &temp_000124)?;
            temp_000119
        };
        let temp_000125 = {
            let temp_000126 = r1cs::allocate_number(system.namespace(|| "temp_000126"), "3")?;
            temp_000126
        };
        let temp_000127 = r1cs::allocate_boolean(system.namespace(|| "temp_000127"), true)?;
        let temp_000117 = r1cs::conditional(system.namespace(|| "temp_000117"), &temp_000118, &temp_000125, &temp_000127)?;
        let temp_000128 = r1cs::allocate_boolean(system.namespace(|| "temp_000128"), false)?;
        let temp_000114 = r1cs::conditional(system.namespace(|| "temp_000114"), &temp_000115, &temp_000117, &temp_000128)?;
        let branch = temp_000114;
        let temp_000129 = r1cs::allocate_number(system.namespace(|| "temp_000129"), "42")?;
        let temp_000130 = r1cs::equals_number(system.namespace(|| "temp_000130"), &branch, &temp_000129, 8)?;
        r1cs::require(system.namespace(|| "L217C1"), &temp_000130, "L217C1");
        let temp_000131 = r1cs::allocate_number(system.namespace(|| "temp_000131"), "0")?;
        let mut value_1: AllocatedNum<Bn256> = temp_000131;
        let temp_000132 = r1cs::allocate_number(system.namespace(|| "temp_000132"), "1")?;
        let mut value_2: AllocatedNum<Bn256> = temp_000132;
        let mut fibo = value_1.clone();
        for l_index in 1..=6 {
            let l = r1cs::allocate_number(system.namespace(|| format!("temp_000132_{}", l_index)), l_index.to_string().as_str())?;
            let temp_000133 = r1cs::add(system.namespace(|| format!("temp_000133_{}", l_index)), &value_1, &value_2, 254)?.0;
            fibo = temp_000133;
            value_1 = value_2.clone();
            value_2 = fibo.clone();
        }
        let temp_000134 = r1cs::allocate_number(system.namespace(|| "temp_000134"), "13")?;
        let temp_000135 = r1cs::cast(system.namespace(|| "temp_000135"), &temp_000134, 254)?;
        let temp_000136 = r1cs::equals_number(system.namespace(|| "temp_000136"), &fibo, &temp_000135, 254)?;
        r1cs::require(system.namespace(|| "L235C1"), &temp_000136, "L235C1");
        let temp_000137 = r1cs::allocate_number(system.namespace(|| "temp_000137"), "1")?;
        let mut fact: AllocatedNum<Bn256> = temp_000137;
        for m_index in 2..6 {
            let m = r1cs::allocate_number(system.namespace(|| format!("temp_000137_{}", m_index)), m_index.to_string().as_str())?;
            let temp_000138 = r1cs::cast(system.namespace(|| format!("temp_000138_{}", m_index)), &m, 254)?;
            let temp_000139 = r1cs::multiply(system.namespace(|| format!("temp_000139_{}", m_index)), &fact, &temp_000138, 254)?.0;
            fact = temp_000139;
        }
        let temp_000140 = r1cs::allocate_number(system.namespace(|| "temp_000140"), "120")?;
        let temp_000141 = r1cs::cast(system.namespace(|| "temp_000141"), &temp_000140, 254)?;
        let temp_000142 = r1cs::equals_number(system.namespace(|| "temp_000142"), &fact, &temp_000141, 254)?;
        r1cs::require(system.namespace(|| "L247C1"), &temp_000142, "L247C1");
        struct MyStruct { data: AllocatedNum<Bn256> }
        let temp_000143 = r1cs::allocate_number(system.namespace(|| "temp_000143"), "42")?;
        let temp_000144 = MyStruct { data: temp_000143 };
        let mut payload = temp_000144;
        let temp_000145 = r1cs::allocate_number(system.namespace(|| "temp_000145"), "1")?;
        let temp_000146 = r1cs::allocate_number(system.namespace(|| "temp_000146"), "2")?;
        let temp_000147 = r1cs::allocate_number(system.namespace(|| "temp_000147"), "3")?;
        let temp_000148 = [temp_000145, temp_000146, temp_000147];
        let temp_000149 = r1cs::allocate_number(system.namespace(|| "temp_000149"), "4")?;
        let temp_000150 = r1cs::allocate_number(system.namespace(|| "temp_000150"), "5")?;
        let temp_000151 = r1cs::allocate_number(system.namespace(|| "temp_000151"), "6")?;
        let temp_000152 = [temp_000149, temp_000150, temp_000151];
        let temp_000153 = [temp_000148, temp_000152];
        let mega_array = temp_000153;
        let temp_000155 = {
            let temp_000156 = r1cs::allocate_number(system.namespace(|| "temp_000156"), "1")?;
            let temp_000157 = r1cs::allocate_number(system.namespace(|| "temp_000157"), "1")?;
            let temp_000158 = r1cs::add(system.namespace(|| "temp_000158"), &temp_000157, &temp_000156, 254)?.0;
            let temp_000159 = r1cs::allocate_number(system.namespace(|| "temp_000159"), "7")?;
            let temp_000160 = r1cs::subtract(system.namespace(|| "temp_000160"), &temp_000159, &temp_000158, 254)?.0;
            let temp_000161 = r1cs::allocate_number(system.namespace(|| "temp_000161"), "15")?;
            let temp_000162 = r1cs::add(system.namespace(|| "temp_000162"), &temp_000161, &temp_000160, 254)?.0;
            let temp_000163 = r1cs::allocate_number(system.namespace(|| "temp_000163"), "3")?;
            let temp_000164 = r1cs::multiply(system.namespace(|| "temp_000164"), &temp_000162, &temp_000163, 254)?.0;
            let temp_000165 = r1cs::add(system.namespace(|| "temp_000165"), &(payload.data), &((mega_array[1])[1]), 254)?.0;
            let temp_000166 = r1cs::allocate_number(system.namespace(|| "temp_000166"), "2")?;
            let temp_000167 = r1cs::add(system.namespace(|| "temp_000167"), &temp_000166, &temp_000165, 254)?.0;
            let temp_000168 = r1cs::subtract(system.namespace(|| "temp_000168"), &temp_000164, &temp_000167, 254)?.0;
            let r = temp_000168;
            let temp_000169 = r1cs::allocate_number(system.namespace(|| "temp_000169"), "11")?;
            let temp_000170 = r1cs::equals_number(system.namespace(|| "temp_000170"), &r, &temp_000169, 8)?;
            r1cs::require(system.namespace(|| "L266C5"), &temp_000170, "L266C5");
        };
        let temp_000171 = {
            let temp_000172 = r1cs::allocate_number(system.namespace(|| "temp_000172"), "50")?;
            (payload.data) = temp_000172;
            let temp_000173 = r1cs::allocate_number(system.namespace(|| "temp_000173"), "5")?;
            let temp_000174 = r1cs::add(system.namespace(|| "temp_000174"), &(payload.data), &temp_000173, 254)?.0;
            let temp_000175 = r1cs::allocate_number(system.namespace(|| "temp_000175"), "55")?;
            let temp_000176 = r1cs::equals_number(system.namespace(|| "temp_000176"), &temp_000174, &temp_000175, 8)?;
            r1cs::require(system.namespace(|| "L269C5"), &temp_000176, "L269C5");
        };
        let temp_000177 = r1cs::allocate_number(system.namespace(|| "temp_000177"), "1")?;
        let temp_000178 = r1cs::equals_number(system.namespace(|| "temp_000178"), &((mega_array[0])[0]), &temp_000177, 8)?;
        Ok(())
    }
}
