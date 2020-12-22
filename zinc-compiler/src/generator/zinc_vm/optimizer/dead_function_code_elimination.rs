//!
//! The bytecode dead function code eliminator.
//!

use std::collections::HashMap;

use petgraph::algo::DfsSpace;
use petgraph::graph::Graph;

use zinc_types::Instruction;

///
/// The dead function code elimination optimization.
///
/// Usually there are several entry points to an application. Each entry ends up with its own
/// call graph, so we can remove the unused functions from the bytecode.
///
pub struct Optimizer {}

impl Optimizer {
    ///
    /// The algorithm works as follows:
    ///
    /// 1. Initialize a call graph and create the function ID to graph node ID mapping.
    ///
    /// 2. Populate the call graph for the current entry. The difference in the graph is caused by
    /// the first `Call` instruction in the bytecode, which specifies the entry that must be called
    /// at the beginning of the execution.
    ///
    /// 3. Replace all the unused function code with `NoOperation` instructions, record the number
    /// of replaced instructions in order to shift the addresses of functions which appear later
    /// in the bytecode.
    ///
    /// 4. Filter out the `NoOperation` instructions placed in the previous step.
    ///
    /// 5. Replace the function type IDs in `Call` instructions with their shifted addresses.
    ///
    pub fn optimize(
        entry_ids: Vec<usize>,
        mut instructions: &mut Vec<Instruction>,
        mut function_addresses: &mut HashMap<usize, usize>,
    ) {
        let mut graph = Graph::<(), ()>::new();
        let mut function_node_map = HashMap::with_capacity(function_addresses.len());
        for (function_id, _) in function_addresses.iter() {
            let function_node = graph.add_node(());
            function_node_map.insert(*function_id, function_node);
        }

        for (caller_id, start_address) in function_addresses.iter() {
            let caller_node = function_node_map
                .get(caller_id)
                .copied()
                .expect(zinc_const::panic::VALIDATED_DURING_TARGET_CODE_GENERATION);

            for address in *start_address..instructions.len() {
                match instructions.get(address) {
                    Some(Instruction::Call(zinc_types::Call {
                        address: callee_id, ..
                    })) => {
                        let callee_node = function_node_map
                            .get(callee_id)
                            .copied()
                            .expect(zinc_const::panic::VALIDATED_DURING_TARGET_CODE_GENERATION);

                        graph.update_edge(caller_node, callee_node, ());
                    }
                    Some(Instruction::Return(_)) => break,
                    _ => {}
                }
            }
        }

        let mut graph_workspace = DfsSpace::new(&graph);
        let mut function_address_shifts =
            HashMap::<usize, usize>::with_capacity(function_addresses.len());
        for (type_id, start_address) in function_addresses.iter() {
            let mut is_isolated = true;
            for entry_id in entry_ids.iter() {
                if petgraph::algo::has_path_connecting(
                    &graph,
                    function_node_map
                        .get(entry_id)
                        .copied()
                        .expect(zinc_const::panic::VALIDATED_DURING_TARGET_CODE_GENERATION),
                    function_node_map
                        .get(type_id)
                        .copied()
                        .expect(zinc_const::panic::VALIDATED_DURING_TARGET_CODE_GENERATION),
                    Some(&mut graph_workspace),
                ) {
                    is_isolated = false;
                    break;
                }
            }

            if is_isolated {
                let mut removed_count = 0;
                for address in *start_address..instructions.len() {
                    let is_end = matches!(instructions.get(address), Some(Instruction::Return(_)));

                    instructions[address] = Instruction::NoOperation(zinc_types::NoOperation);
                    removed_count += 1;

                    if is_end {
                        break;
                    }
                }

                for (type_id, shifted_address) in function_addresses.iter() {
                    if shifted_address > start_address {
                        function_address_shifts
                            .entry(*type_id)
                            .and_modify(|value| *value += removed_count)
                            .or_insert(removed_count);
                    }
                }
            }
        }

        instructions.retain(|instruction| !matches!(instruction, Instruction::NoOperation(_)));
        Self::set_shifted_call_addresses(
            &mut instructions,
            &mut function_addresses,
            &function_address_shifts,
        );
    }

    ///
    /// Replaces the function type IDs in `Call` instructions with their addresses.
    ///
    pub fn set_addresses(
        instructions: &mut Vec<Instruction>,
        function_addresses: &HashMap<usize, usize>,
    ) {
        for instruction in instructions.iter_mut() {
            if let Instruction::Call(zinc_types::Call {
                address: ref mut type_id,
                ..
            }) = instruction
            {
                *type_id = function_addresses
                    .get(&type_id)
                    .copied()
                    .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);
            }
        }
    }

    ///
    /// Replaces the function type IDs in `Call` instructions with their addresses,
    /// shifted after the dead function code elimination.
    ///
    fn set_shifted_call_addresses(
        instructions: &mut Vec<Instruction>,
        function_addresses: &mut HashMap<usize, usize>,
        function_address_shifts: &HashMap<usize, usize>,
    ) {
        for (type_id, address) in function_addresses.iter_mut() {
            if let Some(shift) = function_address_shifts.get(type_id) {
                *address -= shift;
            }
        }

        Self::set_addresses(instructions, function_addresses)
    }
}
