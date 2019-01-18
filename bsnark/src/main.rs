extern crate rand;
extern crate pairing;
extern crate bellman;

mod circuits;
// mod snark;

use crate::circuits::prod_circuit::ProdCircuit;
// use crate::snark::Snark;

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


fn main() {
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

    // Printing random input
    println!("a = {}, b = {}, c = {}", a, b, c);

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


    /* Desirable interface */
    // let snark = Snark::new();

    // // Trusint setup
    // let circuit_none = ProdCircuit { a: None, b: None };
    // let pk, vk = snark.trusting_setup(&circuit_none);

    // // Generating proof
    // let circuit_custom = ProdCircuit {
    //     a: Fr::rand(rng),
    //     b: Fr::rand(rng)
    // };
    // let proof = snark.create_proof(circuit_custom, pk);

    // // Verifying
    // let mut c = circuit_custom.a;
    // c.mul_assign(circuit_custom.b);
    // let verified = snark.verify(vk, proof, vec![c]);
    // assert!(verified).unwrap();
}
