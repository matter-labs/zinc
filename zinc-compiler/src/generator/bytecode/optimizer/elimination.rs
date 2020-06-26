//!
//! The bytecode dead function code eliminator.
//!

use std::collections::HashMap;
use std::collections::HashSet;

use zinc_bytecode::Instruction;

///
/// The dead function code elimination optimization.
///
/// Usually there are several entry points to an application. Each entry ends up with its own
/// call graph, so we can remove the unused functions from the bytecode.
///
pub struct Elimination {
    pub output: Vec<Instruction>,
}

impl Elimination {
    ///
    /// The algorithm works as follows:
    ///
    /// 1. Create the set of functions called for the current entry. The difference is created by
    /// the first `Call` instruction in the bytecode, which specifies the entry that must be called
    /// at the beginning of the execution.
    ///
    /// 2. Replace all the unused function code with `NoOperation` instructions, record the number
    /// of replaced instructions in order to shift the addresses of functions which appear later
    /// in the bytecode.
    ///
    /// 3. Filter out the `NoOperation` instructions placed in the previous step.
    ///
    /// 4. Replace the function type IDs in `Call` instructions with their shifted addresses.
    ///
    pub fn optimize(
        function_addresses: HashMap<usize, usize>,
        mut instructions: Vec<Instruction>,
    ) -> Self {
        let mut called_function_ids = HashSet::with_capacity(function_addresses.len());
        for instruction in instructions.iter_mut() {
            if let Instruction::Call(zinc_bytecode::Call {
                address: ref mut type_id,
                ..
            }) = instruction
            {
                called_function_ids.insert(*type_id);
            }
        }

        let mut function_address_shifts = HashMap::with_capacity(function_addresses.len());
        for (unique_id, start_address) in function_addresses.iter() {
            if !called_function_ids.contains(unique_id) {
                let mut removed_count = 0;
                for address in *start_address.. {
                    let is_end = match instructions.get(address) {
                        Some(Instruction::Return(_)) => true,
                        _ => false,
                    };

                    instructions[address] = Instruction::NoOperation(zinc_bytecode::NoOperation);
                    removed_count += 1;

                    if is_end {
                        break;
                    }
                }

                for (unique_id, shifted_address) in function_addresses.iter() {
                    if shifted_address > start_address {
                        function_address_shifts
                            .entry(unique_id)
                            .and_modify(|value| *value += removed_count)
                            .or_insert(removed_count);
                    }
                }
            }
        }

        let mut instructions: Vec<Instruction> = instructions
            .into_iter()
            .filter(|instruction| !matches!(instruction, Instruction::NoOperation(_)))
            .collect();

        for instruction in instructions.iter_mut() {
            if let Instruction::Call(zinc_bytecode::Call {
                address: ref mut type_id,
                ..
            }) = instruction
            {
                let shift = function_address_shifts
                    .get(type_id)
                    .copied()
                    .unwrap_or_default();

                *type_id = function_addresses
                    .get(&type_id)
                    .copied()
                    .expect(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                    - shift;
            }
        }

        Self {
            output: instructions,
        }
    }
}
