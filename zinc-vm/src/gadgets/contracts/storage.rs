use crate::gadgets::contracts::merkle_tree_storage::{MerkleTreeLeaf, MerkleTreeStorage};
use crate::gadgets::{IntegerType, Scalar, ScalarType, ScalarTypeExpectation, ScalarVariant};
use crate::{Engine, Result};
use franklin_crypto::bellman::ConstraintSystem;
use crate::stdlib::bits::ToBits::signed_to_bits;

pub fn load_from_storage<E, CS>(
    mut cs: CS,
    storage: &MerkleTreeStorage<E>,
    root_hash: &Scalar<E>,
    index: &Scalar<E>,
) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    // we believe that index bitlength not greater that storage.depth()


//    let mut index_bits = match index.get_type() {
//        ScalarType::Integer(t) => {
//            if t.is_signed {
//                signed_to_bits(cs.namespace(|| "signed_to_bits"), index)?
//            } else {
//                index.into_bits_le_fixed(cs.namespace(|| "into_bits_le"), t.bitlength)?
//            }
//        }
//        ScalarType::Boolean => vec![scalar.to_boolean(cs.namespace(|| "to_boolean"))?],
//        ScalarType::Field => {
//            expr.into_bits_le_strict(cs.namespace(|| "into_bits_le_strict"))?
//        }
//    };
//
//    // We use big-endian
//    bits.reverse();
}

pub fn save_to_storage<E, CS>(
    mut cs: CS,
    storage: &MerkleTreeStorage<E>,
    root_hash: &Scalar<E>,
    index: &Scalar<E>,
    value: &Scalar<E>,
) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    // we believe that index bitlength not greater that storage.depth()
}
