//!
//! The virtual machine circuit synthesizer.
//!

use std::marker::PhantomData;

use num_bigint::BigInt;

use franklin_crypto::bellman;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::bellman::SynthesisError;

use zinc_bytecode::Program as BytecodeProgram;

use crate::constraint_systems::dedup::DedupCS;
use crate::constraint_systems::logging::LoggingCS;
use crate::core::circuit::Circuit;
use crate::error::RuntimeError;
use crate::IEngine;

pub struct Synthesizer<'a, E: IEngine> {
    pub inputs: Option<Vec<BigInt>>,
    pub output: &'a mut Option<Result<Vec<Option<BigInt>>, RuntimeError>>,
    pub bytecode: BytecodeProgram,

    pub _pd: PhantomData<E>,
}

impl<E> bellman::Circuit<E> for Synthesizer<'_, E>
where
    E: IEngine,
{
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let mut circuit = Circuit::new(DedupCS::new(LoggingCS::new(cs)), false);
        *self.output =
            Some(circuit.run(&self.bytecode, self.inputs.as_deref(), |_| {}, |_| Ok(())));

        Ok(())
    }
}
