use num_bigint::{BigUint};

#[derive(PartialEq, Clone)]
enum Point {
    Coordinate(BigUint, BigUint),
    Identity,
}
struct EllipticCurve {
    // y² = x² + a*x +b
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    pub fn add(&self, j: &Point, k: &Point) -> Point {
        assert!(self.is_on_curve(j), "Point does not belong to curve");
        assert!(self.is_on_curve(k), "Point does not belong to curve");
        assert!(*j != *k, "Points are identical.");
        match (j, k) {
            (Point::Identity,_) => k.clone(),
            (_, Point::Identity) => j.clone(),
            (Point::Coordinate(x1, y1) , Point::Coordinate(x2, y2)) => {
                // s = (y2 -y1) / (x2 -x1)
                // x3 = s² -x1 -x2
                // y³ = s (x1 -x3) -y1 mod p
                let y2minusy1 = (FiniteField::subtract(y2, y1, &self.p));
                let x2minusx1 = (FiniteField::subtract(x2, x1, &self.p));
                let s = FiniteField::divide(&y2minusy1, &x2minusx1, &self.p);
                let s2 = s.modpow(&BigUint::from(2u32), &self.p);
                let s2minusx1 = FiniteField::subtract(&s2, &x1, &self.p);
                let x3 = FiniteField::subtract(&s2minusx1, x2, &self.p);
                let x1minusx3 = FiniteField::subtract(x1, &x3, &self.p);
                let sx1minusx3 = FiniteField::mul(&s, &x1minusx3, &self.p);
                let y3 = FiniteField::subtract(&sx1minusx3, &y1, &self.p);
                Point::Coordinate(x3, y3)



            }
        }
    }

    fn double(j: &Point) -> Point {
        todo!()
    }


    fn scalar_mul(j: &Point) -> Point {
        //addition/doubling algorithm
        // B = g* A
        todo!()
    }


    fn is_on_curve(&self, j: &Point) -> bool {
        // y² = x³ + a*x + b
        match j {
            Point::Coordinate(x, y ) => {
                let y2 = y.modpow (&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = &self.a * x;
                y2 == x3 +ax + &self.b
            }

            Point::Identity => true,
        } 

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


    pub fn subtract(j: &BigUint, k: &BigUint, p: &BigUint) -> BigUint {
        let k_inv = FiniteField::inv_addition(k, p);
        FiniteField::add(j, &k_inv, p)
    }

    pub fn inv_multiplication(j: &BigUint, p: &BigUint) -> BigUint {
        // TODO: this function is only valid for a p prime
        // c^(-1) mod p = c^(p-2) mod p 
        j.modpow(&(p - BigUint::from(2u32)), p)
    }

    pub fn divide(j: &BigUint, k: &BigUint, p: &BigUint) -> BigUint {
        let k_inv = FiniteField::inv_multiplication(k, p);
        FiniteField::add (j, &k_inv, p)
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