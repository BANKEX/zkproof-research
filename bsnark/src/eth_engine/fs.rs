use std::{
    convert,
    fmt
};

// use ff::{
//     Field,
//     PrimeField,
//     SqrtField,
//     PrimeFieldRepr
// };


pub struct Fs {
    _arr: [u64; 4]
}


// MODULUS = 21888242871839275222246405745257275088696311157297823662689037894645226208583
// const MODULUS: Fs = Fs([0x3c208c16d87cfd47, 0x97816a916871ca8d, 0xb85045b68181585d, 0x30644e72e131a029]);


impl Fs {
    fn new(arr: [u64; 4]) -> Self {
        Self {
            _arr: arr.clone()
        }
    }
}


impl fmt::Display for Fs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:016X}-{:016X}-{:016X}-{:016X}", self._arr[3], self._arr[2], self._arr[1], self._arr[0])
    }
}


impl convert::From<u64> for Fs {
    fn from(a: u64) -> Self {
        Self {
            _arr: [a, 0, 0, 0]
        }
    }
}


impl convert::From<Fs> for u64 {
    fn from(fs: Fs) -> u64 {
        fs._arr[0]
    }
}


// impl Field for Fs {
//     // ...
// }


// impl PrimeField for Fs {
//     // ...
// }


// impl PrimeFieldRepr for Fs {
//     // ..
// }


// impl SqrtField for Fs {
//     // ...
// }


#[cfg(test)]
mod tests {
    /*
    cargo test fs -- --nocapture
    */

    use super::Fs;

    #[test]
    fn test_new() {
        let x = Fs::new([1, 2, 3, 54]);
        assert!(u64::from(x) == 1);
    }

    #[test]
    fn test_from() {
        let x = Fs::from(42);
        assert!(u64::from(x) == 42);
    }
}
