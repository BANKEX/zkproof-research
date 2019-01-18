use pairing::{
    Field,
    Engine
};

use bellman::{
    Circuit,
    SynthesisError,
    ConstraintSystem
};


pub struct ProdCircuit<E: Engine> {
    pub a: Option<E::Fr>,
    pub b: Option<E::Fr>
}


impl<E: Engine> Circuit<E> for ProdCircuit<E> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        let a = cs.alloc(|| "a", || self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.alloc(|| "b", || self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.alloc_input(|| "c", || {
            let mut a = self.a.ok_or(SynthesisError::AssignmentMissing)?;
            let b = self.b.ok_or(SynthesisError::AssignmentMissing)?;

            a.mul_assign(&b);
            Ok(a)
        })?;

        cs.enforce(
            || "a*b=c",
            |lc| lc + a,
            |lc| lc + b,
            |lc| lc + c
        );

        Ok(())
    }
}
