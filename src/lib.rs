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
    pub fn add(j: &BigUint, k: &BigUint, p: &BigUint) -> BigUint {
        // j + k = r mod p
        let r = j + k;
        r.modpow(&BigUint::from(1u32), p)
    }

    pub fn mul(j: &BigUint, k: &BigUint, p: &BigUint) -> BigUint {
        // j + k = r mod p
        let r = j * k;
        r.modpow(&BigUint::from(1u32), p)
    }

   pub fn inv_addition(j: &BigUint, p: &BigUint) -> BigUint {
        // -j mod p "number {} is bigger or equal than p: {}", j , p
        assert!(
            j < p,
            "j >= p" 
        );
        p - j
    }

    pub fn inv_multiplication(j: &BigUint, p: &BigUint) -> BigUint {
        // TODO: this function is only valid for a p prime
        // c^(-1) mod p = c^(p-2) mod p 
        j.modpow(&(p - BigUint::from(2u32)), p)
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

    #[test]
    fn test_mul_1() {
        let j = BigUint::from(4u32);
        let k = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let r = FiniteField::mul(&j, &k, &p);

        assert_eq!(r, BigUint::from(7u32));
    }


    #[test]
    fn test_mul_2() {
        let j = BigUint::from(4u32);
        let k = BigUint::from(10u32);
        let p = BigUint::from(51u32);

        let r = FiniteField::mul(&j, &k, &p);

        assert_eq!(r, BigUint::from(40u32));
    }

    #[test]
    fn test_inv_addition_1() {
        let j = BigUint::from(4u32);
        let p = BigUint::from(51u32);

        let r = FiniteField::inv_addition(&j, &p);

        assert_eq!(r, BigUint::from(47u32));
    }

    #[test]
    #[should_panic]
    fn test_inv_addition_2() {
        let j = BigUint::from(52u32);
        let p = BigUint::from(51u32);

        let r = FiniteField::inv_addition(&j, &p);

    }

    #[test]
    fn test_inv_addition_check() {
        let j = BigUint::from(4u32);
        let p = BigUint::from(51u32);

        let j_inv = FiniteField::inv_addition(&j, &p);

        assert_eq!(j_inv, BigUint::from(47u32));
        assert_eq!(FiniteField::add(&j, &j_inv, &p), BigUint::from(0u32));

    }

    #[test]
    fn test_inv_multiplication_check() {
        let j = BigUint::from(4u32);
        let p = BigUint::from(11u32);

        let j_inv = FiniteField::inv_multiplication(&j, &p);

        // 4 * 3 mod 11 = 12 mod 11 = 1
        assert_eq!(j_inv, BigUint::from(3u32));
        assert_eq!(FiniteField::mul(&j, &j_inv, &p), BigUint::from(1u32));

    }

}