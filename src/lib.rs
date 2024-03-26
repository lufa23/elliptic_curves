use num_bigint::{BigUint};

struct Point {
    x: BigUint,
    y: BigUint,
}
struct EllipticCurve {
    // y² = x² + a*x +b
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    fn add(j: &Point, k: &Point) -> Point {
        todo!()
    }

    fn double(j: &Point) -> Point {
        todo!()
    }


    fn scalar_mul(j: &Point) -> Point {
        //addition/doubling algorithm
        // B = g* A
        todo!()
    }

}


struct FiniteField {}

impl FiniteField {
    fn add(j: &Point, k: &Point, p: &BigUint) -> BigUint {
        // j + k = r mod p
        todo!()
    }

    fn mul(j: &Point, k: &Point, p: &BigUint) -> BigUint {
        // j + k = r mod p
        todo!()
    }

    fn inv_addition(j: &Point, p: &BigUint) -> BigUint {
        // -j mod p
        todo!()
    }

    fn inv_multiplication(j: &Point, p: &BigUint) -> BigUint {
        // // c⁽-1) mod p
        todo!()
    }


}