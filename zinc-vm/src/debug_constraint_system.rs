use ff::Field;
use franklin_crypto::bellman::{
    ConstraintSystem, Index, LinearCombination, SynthesisError, Variable,
};

use crate::Engine;

pub struct DebugConstraintSystem<E: Engine> {
    inputs: Vec<E::Fr>,
    witness: Vec<E::Fr>,

    satisfied: bool,
    constraints_num: usize,
}

impl<E: Engine> Default for DebugConstraintSystem<E> {
    fn default() -> Self {
        let mut cs = Self {
            inputs: Vec::new(),
            witness: Vec::new(),
            satisfied: true,
            constraints_num: 0,
        };

        cs.inputs.push(E::Fr::one());
        cs
    }
}

impl<E: Engine> DebugConstraintSystem<E> {
    pub fn is_satisfied(&self) -> bool {
        self.satisfied
    }

    pub fn num_constraints(&self) -> usize {
        self.constraints_num
    }
}

impl<E: Engine> ConstraintSystem<E> for DebugConstraintSystem<E> {
    type Root = Self;

    fn one() -> Variable {
        Variable::new_unchecked(Index::Input(0))
    }

    fn alloc<F, A, AR>(&mut self, _annotation: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        let value = f()?;
        self.witness.push(value);
        Ok(Variable::new_unchecked(Index::Aux(self.witness.len() - 1)))
    }

    fn alloc_input<F, A, AR>(&mut self, _annotation: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: Into<String>,
    {
        let value = f()?;
        self.inputs.push(value);
        Ok(Variable::new_unchecked(Index::Input(self.inputs.len() - 1)))
    }

    fn enforce<A, AR, LA, LB, LC>(&mut self, _annotation: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: Into<String>,
        LA: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
        LB: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
        LC: FnOnce(LinearCombination<E>) -> LinearCombination<E>,
    {
        let zero = LinearCombination::zero();
        let value_a = eval_lc::<E>(a(zero.clone()).as_ref(), &self.inputs, &self.witness);
        let value_b = eval_lc::<E>(b(zero.clone()).as_ref(), &self.inputs, &self.witness);
        let value_c = eval_lc::<E>(c(zero).as_ref(), &self.inputs, &self.witness);

        let value_ab = {
            let mut tmp: E::Fr = value_a;
            tmp.mul_assign(&value_b);
            tmp
        };

        if value_ab != value_c {
            dbg!(value_a, value_b, value_c);
            self.satisfied = false;
        }

        self.constraints_num += 1;
    }

    fn push_namespace<NR, N>(&mut self, _name_fn: N)
    where
        NR: Into<String>,
        N: FnOnce() -> NR,
    {
    }

    fn pop_namespace(&mut self) {}

    fn get_root(&mut self) -> &mut Self::Root {
        self
    }
}

fn eval_lc<E: Engine>(terms: &[(Variable, E::Fr)], inputs: &[E::Fr], witness: &[E::Fr]) -> E::Fr {
    let mut acc = E::Fr::zero();

    for &(var, ref coeff) in terms {
        let mut tmp = match var.get_unchecked() {
            Index::Input(index) => inputs[index],
            Index::Aux(index) => witness[index],
        };

        tmp.mul_assign(&coeff);
        acc.add_assign(&tmp);
    }

    acc
}
