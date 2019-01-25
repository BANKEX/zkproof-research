use pairing::{
    Field,
    Engine
};

use bellman::{
    Circuit,
    SynthesisError,
    ConstraintSystem
};


pub struct PedersenHashCircuit<E: Engine> {
    pub bits: [Boolean]
}


impl<E: Engine> Circuit<E> for PedersenHashCircuit<E> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        let bits = cs.alloc(|| "bits", || self.bits.ok_or(SynthesisError::AssignmentMissing))?;

        let hash = cs.alloc_input(|| "hash", || {
            let bits = cs.alloc(|| "bits", || self.bits.ok_or(SynthesisError::AssignmentMissing))?;
            let hash = pedersen_hash(cs, bits);  // TODO: params?
            Ok(hash)
        })?;

        cs.enforce(
            || "pedersen hash",
            |lc| lc + bits,
            |lc| lc + CS::one(),
            |lc| lc + hash
        );

        Ok(())
    }
}


pub fn pedersen_hash<E: Engine, CS>(
    mut cs: CS,
    // personalization: Personalization,   -> ???
    bits: &[Boolean],
    // params: &E::Params   -> ???
) -> Result<EdwardsPoint<E>, SynthesisError>
    where CS: ConstraintSystem<E>
{
    let personalization = personalization.get_constant_bools();
    assert_eq!(personalization.len(), 6);

    let mut edwards_result = None;
    let mut bits = personalization.iter().chain(bits.iter());
    let mut segment_generators = params.pedersen_circuit_generators().iter();
    let boolean_false = Boolean::constant(false);

    let mut segment_i = 0;
    loop {
        let mut segment_result = None;
        let mut segment_windows = &segment_generators.next()
                                                     .expect("enough segments")[..];

        let mut window_i = 0;
        while let Some(a) = bits.next() {
            let b = bits.next().unwrap_or(&boolean_false);
            let c = bits.next().unwrap_or(&boolean_false);

            let tmp = lookup3_xy_with_conditional_negation(
                cs.namespace(|| format!("segment {}, window {}", segment_i, window_i)),
                &[a.clone(), b.clone(), c.clone()],
                &segment_windows[0]
            )?;

            let tmp = MontgomeryPoint::interpret_unchecked(tmp.0, tmp.1);

            match segment_result {
                None => {
                    segment_result = Some(tmp);
                },
                Some(ref mut segment_result) => {
                    *segment_result = tmp.add(
                        cs.namespace(|| format!("addition of segment {}, window {}", segment_i, window_i)),
                        segment_result,
                        params
                    )?;
                }
            }

            segment_windows = &segment_windows[1..];

            if segment_windows.len() == 0 {
                break;
            }

            window_i += 1;
        }

        match segment_result {
            Some(segment_result) => {
                // Convert this segment into twisted Edwards form.
                let segment_result = segment_result.into_edwards(
                    cs.namespace(|| format!("conversion of segment {} into edwards", segment_i)),
                    params
                )?;

                match edwards_result {
                    Some(ref mut edwards_result) => {
                        *edwards_result = segment_result.add(
                            cs.namespace(|| format!("addition of segment {} to accumulator", segment_i)),
                            edwards_result,
                            params
                        )?;
                    },
                    None => {
                        edwards_result = Some(segment_result);
                    }
                }
            },
            None => {
                // We didn't process any new bits.
                break;
            }
        }

        segment_i += 1;
    }

    Ok(edwards_result.unwrap())
}
