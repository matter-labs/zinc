//!
//! The bytecode dead function code eliminator.
//!

use std::collections::HashMap;

use petgraph::algo::DfsSpace;
use petgraph::graph::Graph;

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
        entry_id: usize,
        function_addresses: HashMap<usize, usize>,
        mut instructions: Vec<Instruction>,
    ) -> Self {
        let mut graph = Graph::new();
        let mut function_node_map = HashMap::with_capacity(function_addresses.len());
        for (function_id, _) in function_addresses.iter() {
            let function_node = graph.add_node(1);
            function_node_map.insert(*function_id, function_node);
        }

        for (caller_id, start_address) in function_addresses.iter() {
            let caller_node = function_node_map
                .get(caller_id)
                .copied()
                .expect(crate::panic::VALIDATED_DURING_BYTECODE_GENERATION);

            for address in *start_address.. {
                match instructions.get(address) {
                    Some(Instruction::Call(zinc_bytecode::Call {
                        address: callee_id, ..
                    })) => {
                        let callee_node = function_node_map
                            .get(callee_id)
                            .copied()
                            .expect(crate::panic::VALIDATED_DURING_BYTECODE_GENERATION);

                        graph.update_edge(caller_node, callee_node, 1);
                    }
                    Some(Instruction::Return(_)) => break,
                    _ => {}
                }
            }
        }

        let mut graph_workspace = DfsSpace::new(&graph);
        let mut function_address_shifts = HashMap::with_capacity(function_addresses.len());
        for (unique_id, start_address) in function_addresses.iter() {
            if !petgraph::algo::has_path_connecting(
                &graph,
                function_node_map
                    .get(&entry_id)
                    .copied()
                    .expect(crate::panic::VALIDATED_DURING_BYTECODE_GENERATION),
                function_node_map
                    .get(unique_id)
                    .copied()
                    .expect(crate::panic::VALIDATED_DURING_BYTECODE_GENERATION),
                Some(&mut graph_workspace),
            ) {
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
