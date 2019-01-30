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


#[cfg(test)]
mod tests {
    use rand::{
        Rand,
        thread_rng
    };

    use pairing::Field;

    use pairing::bls12_381::{
        Bls12,
        Fr
    };

    use bellman::groth16::{
        generate_random_parameters,
        prepare_verifying_key,
        create_random_proof,
        verify_proof
    };

    use ProdCircuit;

    #[test]
    fn test_prod_circuit() {
        // Trusting setup
        let rng = &mut thread_rng();

        let params = generate_random_parameters::<Bls12, _, _>(
            ProdCircuit { a: None, b: None },
            rng
        ).unwrap();

        // Generating verifying key
        let pvk = prepare_verifying_key::<Bls12>(&params.vk);

        // Generating proof
        let a = Fr::rand(rng);
        let b = Fr::rand(rng);
        let mut c = a;
        c.mul_assign(&b);

        let proof = create_random_proof(
            ProdCircuit {
                a: Some(a),
                b: Some(b)
            },
            &params,
            rng
        ).unwrap();

        // Verifying proof
        assert!(verify_proof(&pvk, &proof, &[c]).unwrap());
    }
}
