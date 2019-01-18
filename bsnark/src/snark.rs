use rand::{
    // Rand,
    ThreadRng,
    // thread_rng
};

// use pairing::{
//     Field,
//     Engine
// };

// use pairing::bls12_381::{
//     Bls12,
//     Fr
// };

// use bellman::Circuit;

// use bellman::groth16::{
//     Parameters,
//     VerifyingKey,
//     Proof
// };


pub struct Snark {
    rng: Option<ThreadRng>
}


// impl Snark {
//     pub fn new() -> Self {
//         Self { rng: None }
//     }

//     pub fn trusting_setup<E>(&mut self, C: &Circuit<E>) -> (Parameters<E>, VerifyingKey<E>) where E: Engine {
//         self.rng = thread_rng();
//         let params = generate_random_parameters::<Bls12, _, _>(C, self.rng).unwrap();
//         let vk = prepare_verifying_key::<Bls12>(&params.vk);
//         let pk = params;
//         (pk, vk)
//     }

//     pub fn create_proof(&self, C: &Circuit<E>, pk: &Parameters<E>) -> Proof<E> where E: Engine {
//         let proof = create_random_proof(C, &pk, self.rng).unwrap();
//         proof
//     }

//     pub fn verify(&self, vk: &VerifyingKey<E>, proof: &Proof<E>, pi: &Vec<E::Fr>) -> bool where E: Engine {
//         let verified = verify_proof(&vk, &proof, &pi).unwrap();
//         verified
//     }
// }
