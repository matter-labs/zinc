use crate::core::RuntimeError;
use crate::gadgets::comparison::eq;
use crate::gadgets::conditional_select;
use crate::gadgets::contracts::merkle_tree_storage::{MerkleTreeLeaf, MerkleTreeStorage};
use crate::gadgets::{IntegerType, Scalar, ScalarType, ScalarTypeExpectation, ScalarVariant};
use crate::stdlib::bits::signed_to_bits;
use crate::{Engine, Result};
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::{Boolean, AllocatedBit};
use franklin_crypto::circuit::num::AllocatedNum;
use franklin_crypto::circuit::sha256::sha256;
use num_integer::nth_root;
use crate::gadgets::utils::fr_to_bigint_unsigned;
use num_bigint::{BigInt, Sign};
use ff::{Field, PrimeField, PrimeFieldRepr};

pub struct StorageGadget<E: Engine, S: MerkleTreeStorage<E>> {
    storage: S,
    root_hash: Scalar<E>,
}

impl<E: Engine, S: MerkleTreeStorage<E>> StorageGadget<E, S> {
    pub fn new<CS>(mut cs: CS, storage: S) -> Result<Self>
        where
            CS: ConstraintSystem<E>,
    {
        let root_hash_value = storage.root_hash()?;
        println!("root_hash_value :: {:?}", root_hash_value);
        let root_hash_variable = cs
            .alloc_input(|| "root hash variable", || Ok(root_hash_value))
            .map_err(RuntimeError::SynthesisError)?;
        let root_hash = Scalar::<E>::new_unchecked_variable(
            Some(root_hash_value),
            root_hash_variable,
            ScalarType::Field,
        );

        Ok(StorageGadget {
            storage: storage,
            root_hash: root_hash,
        })
    }

    pub fn load<CS>(&self, mut cs: CS, index: &Scalar<E>) -> Result<Vec<Scalar<E>>>
        where
            CS: ConstraintSystem<E>,
    {
        let mut index_bits = index
            .to_expression::<CS>()
            .into_bits_le_strict(cs.namespace(|| "index into bits fixed strict"))?;

        index_bits.truncate(self.storage.depth()? - 1);

        println!("index bits");
        for i in &index_bits {
            print!("{}", i.get_value().unwrap());
        }
        println!("");

        let merkle_tree_leaf = self.storage.load(&fr_to_bigint_unsigned(&index.grab_value().unwrap_or(E::Fr::zero())))?;

        assert_eq!(
            merkle_tree_leaf.authentication_path.len(),
            self.storage.depth()? - 1
        );

        let mut leaf_value: Vec<AllocatedNum<E>> = Vec::new();
        for (index, field) in merkle_tree_leaf.leaf_value.iter().enumerate() {
            leaf_value.push(
                AllocatedNum::alloc(
                    cs.namespace(|| format!("leaf value: element with index {}", index)),
                    || Ok(*field),
                )
                    .map_err(RuntimeError::SynthesisError)?,
            );
        }

        assert!(!leaf_value.is_empty());

        let mut leaf_value_hash: Vec<Boolean> = Vec::new();

        for (index, field) in leaf_value.iter().enumerate() {
            leaf_value_hash = {

                println!("field :: {:?}", field.get_value().unwrap());

                let mut field_to_bits = field.into_bits_le_strict(
                    cs.namespace(|| format!("to bits strict (element with index {})", index)),
                )?;
                field_to_bits.resize(256, Boolean::Constant(false));

                let mut preimage = Vec::new();
                preimage.append(&mut leaf_value_hash.clone());
                preimage.append(&mut field_to_bits);
                println!("preimage in gadget :: ");
                for i in &preimage {
                    print!("{}", i.get_value().unwrap() as u8);
                }
                println!("");
                let mut result = sha256(
                    cs.namespace(|| {
                        format!("adding element with index {} to leaf value hash", index)
                    }),
                    preimage.as_ref(),
                )?;
                result
            };
        }

        let mut current_hash = leaf_value_hash;

//        println!("authentication_path in gadget :: {:?}", merkle_tree_leaf.authentication_path.iter().map(|a| a.iter().map(|b| *b as u8).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>());

        for (index, (vertex_hash_value, bit)) in merkle_tree_leaf
            .authentication_path
            .iter()
            .zip(index_bits)
            .enumerate()
            {
                println!("current hash at the start :: ");
                for i in &current_hash {
                    print!("{}", i.get_value().unwrap() as u8);
                }
                println!("");
                let bit_scalar = Scalar::<E>::from_boolean(
                    cs.namespace(|| format!("index bit with index {} to scalar", index)),
                    bit,
                )?;
                let mut vertex_hash = vec![];
                for (id, bit) in vertex_hash_value.iter().enumerate() {
                    let vertex_hash_boolean = Boolean::from(AllocatedBit::alloc(
                        cs.namespace(|| format!("bit with index {} in auth path vertex (deep equals {})", id, index)),
                        Some(*bit),
                    )?);
                    vertex_hash.push(
                        Scalar::<E>::from_boolean(
                            cs.namespace(|| format!("bit with index {} in auth path vertex (deep equals {}) to boolean", id, index)),
                            vertex_hash_boolean,
                        )?
                    );
                }

                let mut hash_preimage = vec![Boolean::Constant(false); 512];

                for (bit_id, (current_hash_bit, vertex_hash_bit_scalar)) in current_hash.iter().zip(&vertex_hash).enumerate() {
                    let current_hash_bit_scalar = Scalar::<E>::from_boolean(
                        cs.namespace(|| format!("current_hash_bit to scalar {} {}", index, bit_id)),
                        current_hash_bit.clone()
                    )?;

                    hash_preimage[bit_id] = conditional_select(
                        cs.namespace(|| {
                            format!(
                                "vertex hash left preimage part conditional select {} {}",
                                index,
                                bit_id
                            )
                        }),
                        &bit_scalar,
                        &vertex_hash_bit_scalar,
                        &current_hash_bit_scalar,
                    )?.to_boolean(cs.namespace(|| format!("left part to boolean {} {}", index, bit_id)))?;

                    hash_preimage[256 + bit_id] = conditional_select(
                        cs.namespace(|| {
                            format!(
                                "vertex hash right preimage part conditional select {} {}",
                                index,
                                bit_id
                            )
                        }),
                        &bit_scalar,
                        &current_hash_bit_scalar,
                        &vertex_hash_bit_scalar,
                    )?.to_boolean(cs.namespace(|| format!("right part to boolean {} {}", index, bit_id)))?;
                }
                println!("hash preimage in gadget :: ");
                for i in &hash_preimage {
                    print!("{}", i.get_value().unwrap() as u8);
                }
                println!("");

                current_hash = sha256(
                    cs.namespace(|| format!("vertex hash (deep equals {})", index + 1)),
                    &hash_preimage,
                )?;
                println!("current hash at the end :: ");
                for i in &current_hash {
                    print!("{}", i.get_value().unwrap() as u8);
                }
                println!("");
            }

        current_hash.truncate(248);
        println!("current hash at the end of the end :: ");
        for i in &current_hash {
            print!("{}", i.get_value().unwrap() as u8);
        }
        println!("");

        let current_hash_to_scalar = Scalar::<E>::from(AllocatedNum::<E>::pack_bits_to_element(
            cs.namespace(||
                format!("pack root vertex hash into element")
            ),
            &current_hash,
        )?);

        println!("current_hash_to_scalar :: {:?}", current_hash_to_scalar);

        let root_hash_condition = eq(
            cs.namespace(|| "root hash equals to stored"),
            &current_hash_to_scalar,
            &self.root_hash,
        )?
            .to_boolean(cs.namespace(|| "root hash equals to stored to boolean"))?;
        Boolean::enforce_equal(
            cs.namespace(|| "enforcing that root hash equals to stored"),
            &root_hash_condition,
            &Boolean::Constant(true),
        )?;

        Ok(leaf_value
            .iter()
            .map(|al_num| Scalar::<E>::from(al_num))
            .collect())
    }

    pub fn store<CS>(&mut self, mut cs: CS, index: &Scalar<E>, value: &Vec<Scalar<E>>) -> Result<()>
        where
            CS: ConstraintSystem<E>,
    {
        panic!("esv");
    }

    pub fn root_hash<CS>(&self) -> Result<Scalar<E>>
        where
            CS: ConstraintSystem<E>,
    {
        panic!("esv");
    }
}

#[cfg(test)]
mod tests {
    use crate::{Engine, Result};
    use sha2::{Sha256, Digest};
    use crate::gadgets::contracts::merkle_tree_storage::{MerkleTreeLeaf, MerkleTreeStorage};
    use ff::{Field, PrimeField, PrimeFieldRepr};
    use num_bigint::{BigInt, Sign};
    use num_traits::cast::ToPrimitive;

    pub fn calc_hash<E: Engine>(
        preimage: Vec<u8>,
    ) -> Vec<u8>
    {
        let mut h = Sha256::new();
        h.input(&preimage);
        let result = h.result();

        result.as_slice().to_vec()

//        let mut res = vec![];
//        for i in result.as_slice() {
//            let mut cur_byte: u8 = 0;
//            for j in (0..8) {
//                cur_byte <<= 1;
//                cur_byte += ((i >> j) & 1u8);
//            }
//            res.push(cur_byte);
//        }
//
//        res
    }

    pub fn sha256_of_concat<E: Engine>(
        left: Vec<u8>,
        right: Vec<u8>,
    ) -> Vec<u8>
    {
        let mut preimage: Vec<u8> = vec![];
        preimage.append(&mut left.clone());
        preimage.append(&mut right.clone());
        println!("preimage in dummy :: len({}) :: ", preimage.len());
        for i in &preimage {
            for k in (0..8).rev() {
                print!("{}", (i >> k) & 1u8);
            }
        }
        println!("");
        calc_hash::<E>(preimage)
    }

    pub fn add_Fr_to_hash<E: Engine>(
        left: Vec<u8>,
        right_Fr: E::Fr,
    ) -> Vec<u8>
    {
        let mut right_buf = vec![];
        right_Fr.into_repr().write_le(&mut right_buf).unwrap();
        right_buf.resize(32, 0);

        let mut right = vec![];
//        right = right_buf;
        for i in right_buf {
            let mut cur_byte: u8 = 0;
            for j in (0..8) {
                cur_byte <<= 1;
                cur_byte += ((i >> j) & 1u8);
            }
            right.push(cur_byte);
        }

        sha256_of_concat::<E>(left, right)
    }

    pub fn sha256_of_leaf_value<E: Engine>(
        leaf_value: Vec<E::Fr>
    ) -> Vec<u8>
    {
        let mut res = vec![];
        for i in leaf_value {
            res = add_Fr_to_hash::<E>(res, i);
        }
        res
    }

    struct StorageDummy<E: Engine> {
        depth: usize,
        fields: usize,
        tree: Vec<Vec<u8>>,
        leaf_values: Vec<Vec<E::Fr>>,
    }

    impl<E: Engine> StorageDummy<E> {

        pub fn new(
            depth: usize,
            fields: usize,
        ) -> Self
        {
            assert!(fields != 0);
            let mut result = Self {
                depth: depth,
                fields: fields,
                tree: vec![vec![]; 1 << depth],
                leaf_values: vec![vec![E::Fr::zero(); fields]; 1 << (depth - 1)],
            };
            for i in (1..(1 << depth)).rev() {
                if (i * 2 < (1 << depth)) {
                    result.tree[i] = sha256_of_concat::<E>(result.tree[i * 2].clone(), result.tree[i * 2 + 1].clone());
                }
                else {
//                    result.tree[i] = sha256_of_leaf_value::<E>(vec![E::Fr::from_str(&i.to_string()).unwrap(); fields]);
                    if (fields == 2) {
                        let mut cur_vec = vec![];
                        cur_vec.push(E::Fr::from_str(&i.to_string()).unwrap());
                        cur_vec.push(E::Fr::from_str(&(i+1).to_string()).unwrap());
                        result.tree[i] = sha256_of_leaf_value::<E>(cur_vec.clone());
                        result.leaf_values[i-(1 << (depth - 1))] = cur_vec;
                    }
                    else {
                        result.tree[i] = sha256_of_leaf_value::<E>(vec![E::Fr::from_str(&i.to_string()).unwrap(); fields]);
                        result.leaf_values[i - (1 << (depth - 1))] = vec![E::Fr::from_str(&i.to_string()).unwrap(); fields];
                    }
                }
            }

            println!("tree at 'new' call :: ");
            for (index, i) in result.tree.iter().enumerate() {
                println!("next hash {} :: ", index);
                for j in i {
                    for k in (0..8).rev() {
                        print!("{}", (j >> k) & 1u8);
                    }
                }
                println!("");
            }

            result
        }
    }

    impl<E: Engine> MerkleTreeStorage<E> for StorageDummy<E> {

        fn depth(&self) -> Result<usize>
        {
            Ok(self.depth)
        }

        fn root_hash(&self) -> Result<E::Fr>
        {
            let mut hash_as_buf = self.tree[1].clone();

            hash_as_buf.truncate(248 / 8);
            hash_as_buf.resize(32, 0);

            println!("root hash at 'new' call in gadget :: ");
            for j in &hash_as_buf {
                for k in (0..8).rev() {
                    print!("{}", (j >> k) & 1u8);
                }
            }
            println!("");

            let mut hash_le = vec![];
            for i in &hash_as_buf {
                let mut cur_byte: u8 = 0;
                for j in (0..8) {
                    cur_byte <<= 1;
                    cur_byte += ((i >> j) & 1u8);
                }
                hash_le.push(cur_byte);
            }


            let mut hash_repr = <E::Fr as PrimeField>::Repr::default();
            hash_repr
                .read_le(hash_le.as_slice())
                .unwrap();
            println!("root hash in root_hash call :: {:?}", E::Fr::from_repr(hash_repr).unwrap());
            Ok(E::Fr::from_repr(hash_repr).unwrap())
        }

        fn load(&self, index: &BigInt) -> Result<MerkleTreeLeaf<E>>
        {
            println!("we are in load :: {:?}", index);
            let index = index.to_usize().unwrap();

            let mut result = MerkleTreeLeaf::<E> {
                leaf_value: self.leaf_values[index].clone(),
                authentication_path: vec![],
            };

            let mut cur_vertex = 1;
            for i in (0..self.depth - 1).rev() {
                let next = cur_vertex * 2 + ((index >> i) & 1usize);
                let mut cur_auth_path_vertex_hash = vec![];
                println!("add to auth path vertex with index :: {}", next ^ 1usize);
                for i in &self.tree[next ^ 1usize] {
                    for j in (0..8).rev() {
                        cur_auth_path_vertex_hash.push((i >> j) & 1u8 != 0);
                    }
                }
                result.authentication_path.push(cur_auth_path_vertex_hash);

                cur_vertex = next;
            }

            result.authentication_path.reverse();

            Ok(result)
        }

        fn store(
            &mut self,
            index: &BigInt,
            value: &Vec<E::Fr>,
        ) -> Result<(E::Fr, MerkleTreeLeaf<E>)>
        {
            panic!("Awefwr");
        }
    }

    use franklin_crypto::bellman::ConstraintSystem;
    use franklin_crypto::circuit::test::TestConstraintSystem;
    use pairing::bn256::{Bn256, Fr};
    use rand::{XorShiftRng, SeedableRng, Rng};
    use super::StorageGadget;
    use crate::gadgets::{IntegerType, Scalar, ScalarType, ScalarTypeExpectation, ScalarVariant};

    #[test]
    fn test_storage_load()
    {
        let mut cs = TestConstraintSystem::<Bn256>::new();

        let storageDummy = StorageDummy::<Bn256>::new(4, 2);

        let storageGadget = StorageGadget::new(cs.namespace(|| "creation"), storageDummy).unwrap();

        println!("final read 0 :: {:?}",
            storageGadget.load(
                cs.namespace(|| "one access read 0"),
                &Scalar::<Bn256>::new_constant_fr(Fr::from_str("0").unwrap(), ScalarType::Field)
            ).unwrap()
        );
        println!("final read 1 :: {:?}",
             storageGadget.load(
                 cs.namespace(|| "one access read 1"),
                 &Scalar::<Bn256>::new_constant_fr(Fr::from_str("1").unwrap(), ScalarType::Field)
             ).unwrap()
        );
        println!("final read 2 :: {:?}",
             storageGadget.load(
                 cs.namespace(|| "one access read 2"),
                 &Scalar::<Bn256>::new_constant_fr(Fr::from_str("2").unwrap(), ScalarType::Field)
             ).unwrap()
        );
        println!("final read 3 :: {:?}",
             storageGadget.load(
                 cs.namespace(|| "one access read 3"),
                 &Scalar::<Bn256>::new_constant_fr(Fr::from_str("3").unwrap(), ScalarType::Field)
             ).unwrap()
        );
        println!("final read 5 :: {:?}",
             storageGadget.load(
                 cs.namespace(|| "one access read 5"),
                 &Scalar::<Bn256>::new_constant_fr(Fr::from_str("5").unwrap(), ScalarType::Field)
             ).unwrap()
        );

        assert!(cs.is_satisfied());
    }
}
