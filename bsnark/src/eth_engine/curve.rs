use ff::{
    Field
};


pub struct Point<F: Field> {
    x: F,
    y: F,
    isnull: bool
}


pub struct Curve<F: Field> {
    modulus: F,
    params: Vec<F>
}


pub trait CurveGroup<F: Field> {
    fn add(self, p1: &Point<F>, p2: &Point<F>) -> Point<F>;
    fn mul(self, p: &Point<F>, k: F) -> Point<F>;
}


impl<F: Field> CurveGroup<F> for Curve<F> {
    fn add(self, p1: &Point<F>, p2: &Point<F>) -> Point<F> {
        Point::<F> {
            x: F::zero(),
            y: F::zero(),
            isnull: true
        }
    }

    fn mul(self, p: &Point<F>, k: F) -> Point<F> {
        Point::<F> {
            x: F::zero(),
            y: F::zero(),
            isnull: true
        }
    }
}


#[cfg(test)]
mod tests {
    /*
    cargo test curve -- --nocapture
    */

    #[test]
    fn test_new() {
        assert!(true);
    }
}
