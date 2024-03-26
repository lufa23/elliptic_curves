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
    fn add(j: &BigUint, k: &BigUint, p: &BigUint) -> BigUint {
        // j + k = r mod p
        let r = j + k;
        r.modpow(&BigUint::from(1u32), p)
    }

    fn mul(j: &BigUint, k: &BigUint, p: &BigUint) -> BigUint {
        // j + k = r mod p
        todo!()
    }

    fn inv_addition(j: &BigUint, p: &BigUint) -> BigUint {
        // -j mod p
        todo!()
    }

    fn inv_multiplication(j: &BigUint, p: &BigUint) -> BigUint {
        // // c⁽-1) mod p
        todo!()
    }


}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_1() {
        let j = BigUint::from(4u32);
        let k = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let r = FiniteField::add(&j, &k, &p);

        assert_eq!(r, BigUint::from(3u32));
    }
    #[test]
    fn test_add_2() {
        let j = BigUint::from(4u32);
        let k = BigUint::from(10u32);
        let p = BigUint::from(31u32);

        let r = FiniteField::add(&j, &k, &p);

        assert_eq!(r, BigUint::from(14u32));
    }
}