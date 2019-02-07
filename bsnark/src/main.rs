extern crate rand;
extern crate ff;
extern crate pairing;
extern crate bellman;

mod circuits;
mod eth_engine;
// mod snark;

use crate::circuits::prod_circuit::ProdCircuit;
// use crate::snark::Snark;

use std::{
    fmt,
    convert,
    ops,
    cmp,
    marker
};

use rand::{
    Rand,
    Rng,
    thread_rng
};

use ff::{
    adc,
    sbb,
    mac_with_carry
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

// use ff::{
//     PrimeField
// };


#[allow(dead_code)]
fn example_prod_circle_snark() {
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


/* Int256 */

#[derive(Copy, Clone, Eq)]
pub struct Int256(pub [u64; 4]);

impl Int256 {
    fn cmp(&self, other: &Self) -> i32 {
        for i in (0..4).rev() {
            if self.0[i] > other.0[i] {
                return 1;
            }
            if self.0[i] < other.0[i] {
                return-1;
            }
        }
        return 0;
    }

    fn bit(&self, index: usize) -> bool {
        let i = index / 64;
        let b = index & 64;
        (self.0[i] & (1 << b)) != 0
    }
}

impl fmt::Display for Int256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:016X}-{:016X}-{:016X}-{:016X}", self.0[3], self.0[2], self.0[1], self.0[0])
    }
}

impl convert::From<u64> for Int256 {
    fn from(a: u64) -> Int256 {
        Int256([a, 0, 0, 0])
    }
}

impl convert::From<Int256> for u64 {
    fn from(int256: Int256) -> u64 {
        int256.0[0]
    }
}

impl Rand for Int256 {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Int256(rng.gen())
    }
}

impl cmp::PartialEq for Int256 {
    fn eq(&self, other: &Int256) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1] && self.0[2] == other.0[2] && self.0[3] == other.0[3]
    }
}

impl ops::Add<Int256> for Int256 {
    type Output = Int256;

    fn add(self, other: Int256) -> Int256 {
        let mut r = Int256::from(0);
        let mut c: u64 = 0;
        for i in 0..4 {
            r.0[i] = adc(self.0[i], other.0[i], &mut c);
        }
        r
    }
}

impl ops::Sub<Int256> for Int256 {
    type Output = Int256;

    fn sub(self, other: Int256) -> Int256 {
        let mut r = Int256::from(0);
        let mut b: u64 = 0;
        for i in 0..4 {
            r.0[i] = sbb(self.0[i], other.0[i], &mut b);
        }
        r
    }
}

impl ops::Mul<Int256> for Int256 {
    type Output = Int256;

    fn mul(self, other: Int256) -> Int256 {
        let mut r = Int256::from(0);
        for j in 0..4 {
            let mut c: u64 = 0;
            for i in 0..(4 - j) {
                r.0[i + j] = mac_with_carry(r.0[i + j], self.0[i], other.0[j], &mut c);
            }
        }
        r
    }
}


/* Fp */

// MODULUS = 21888242871839275222246405745257275088696311157297823662689037894645226208583
const MODULUS: Int256 = Int256([0x3c208c16d87cfd47, 0x97816a916871ca8d, 0xb85045b68181585d, 0x30644e72e131a029]);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Fp(pub Int256);

impl Fp {
    fn reduce(&mut self) {
        while self.0.cmp(&MODULUS) >= 0 {
            self.0 = self.0 - MODULUS;
        }
    }
}

impl fmt::Display for Fp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Fp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl convert::From<u64> for Fp {
    fn from(a: u64) -> Fp {
        let mut fp = Fp(Int256::from(a));
        fp.reduce();
        fp
    }
}

impl convert::From<Fp> for u64 {
    fn from(fp: Fp) -> u64 {
        u64::from(fp.0)
    }
}

impl Rand for Fp {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut fp = Fp(rng.gen());
        fp.reduce();
        fp
    }
}

impl Field for Fp {
    fn zero() -> Self {
        Fp(Int256::from(0))
    }

    fn one() -> Self {
        Fp(Int256::from(1))
    }

    fn is_zero(&self) -> bool {
        (self.0).0[0] == 0 && (self.0).0[1] == 0 && (self.0).0[2] == 0 && (self.0).0[3] == 0
    }

    fn double(&mut self) {
        let other = self.clone();
        self.add_assign(&other);
    }

    fn negate(&mut self) {
        if !self.is_zero() {
            self.0 = MODULUS - self.0;
        }
    }

    fn add_assign(&mut self, other: &Self) {
        let mut neg_other = other.clone();
        neg_other.negate();
        self.sub_assign(&neg_other);
    }

    fn sub_assign(&mut self, other: &Self) {
        if self.0.cmp(&other.0) != -1 {
            self.0 = self.0 - other.0;
        } else {
            self.0 = other.0 - self.0;
            self.negate();
        }
    }

    fn mul_assign(&mut self, other: &Self) {
        let mut r = Fp::zero();
        let mut a2 = other.clone();
        for i in 0..256 {
            let bit = self.0.bit(i);
            if bit {
                r.add_assign(&a2);
            }
            a2.double();
        }
        *self = r;
    }

    fn square(&mut self) {
        let other = self.clone();
        self.mul_assign(&other);
    }

    fn inverse(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            // TODO: we need some code to inverse the group element
            Some(Self::from(1))
        }
    }

    fn frobenius_map(&mut self, power: usize) {}
}


/* Curve */


fn main() {
    // example_prod_circle_snark();

    let a = Int256::from(0x97816a916871ca8d);
    let b = Int256::from(0x97816a916871ca8d);
    let c = a * b;
    println!("{}", c);

    let mut rng = rand::thread_rng();
    let a = Int256::rand(&mut rng);
    let b = Int256::rand(&mut rng);
    let c = a + b;
    println!("{}\n+\n{}\n=\n{}", a, b, c);

    let mut c: u64 = 0;
    let x = 0x97816a916871ca8d;
    let y = 0x97816a916871ca8d;
    let z = adc(x, y, &mut c);
    println!("{}", z);
    println!("{}", c);

    let q = Fp::from(5);
    println!("{}", q);
    let qi = q.inverse().unwrap();
    println!("{}", qi);

    // let p: u64 = 1;
    // let u = p & (1 << 5);
    // println!("{}", u);
}
