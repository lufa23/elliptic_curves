use num_bigint::BigUint;

#[derive(PartialEq, Clone, Debug)]
enum Point {
    Coordinate(BigUint, BigUint),
    Identity,
}

#[derive(PartialEq, Clone, Debug)]
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
            (Point::Identity, _) => k.clone(),
            (_, Point::Identity) => j.clone(),
            (Point::Coordinate(x1, y1), Point::Coordinate(x2, y2)) => {
                
                let y1plusy2 = FiniteField::add(&y1, &y2, &self.p);
                if x1 == x2 && y1plusy2 == BigUint::from(0u32) {
                    return Point::Identity;
                }
            
                // s = (y2 -y1) / (x2 -x1) mod p
                // x3 = s² -x1 -x2 mod p
                // y³ = s (x1 -x3) -y1 mod p
                let numerator = (FiniteField::subtract(y2, y1, &self.p));
                let denominator = (FiniteField::subtract(x2, x1, &self.p));
                let s = FiniteField::divide(&numerator, &denominator, &self.p);

                let (x3, y3) = self.compute_x3_y3(&x1, &y1, &x2, &s);
                Point::Coordinate(x3, y3)
            }
        }
    }

    fn double(&self, j: &Point) -> Point {
        assert!(self.is_on_curve(j), "Point does not belong to the curve");

        match j {
            Point::Identity => Point::Identity,
            Point::Coordinate(x1, y1) => {

                // s = (3* x1² + a) / (2 * y1) mod p
                // x3 = s² -2*x1 mod p
                // y³ = s (x1 -x3) -y1 mod p

                let numerator = x1.modpow(&BigUint::from(2u32), &self.p);
                let numerator= FiniteField::mul(&BigUint::from(3u32), &numerator, &self.p);
                let numerator= FiniteField::add(&self.a, &numerator, &self.p);

                let denominator: BigUint = FiniteField::mul(&BigUint::from(2u32), y1, &self.p);


                let s = FiniteField::divide(&numerator, &denominator, &self.p);

                let (x3, y3) = self.compute_x3_y3(&x1, &y1, &x1, &s);
                Point::Coordinate(x3, y3)
            }
    }
    


    }

    fn compute_x3_y3(&self, x1: &BigUint, y1: &BigUint, x2: &BigUint, s: &BigUint) -> (BigUint, BigUint) {
        let s2 = s.modpow(&BigUint::from(2u32), &self.p);
        let s2minusx1 = FiniteField::subtract(&s2, &x1, &self.p);
        let x3 = FiniteField::subtract(&s2minusx1, x2, &self.p);
        let x1minusx3 = FiniteField::subtract(x1, &x3, &self.p);
        let sx1minusx3 = FiniteField::mul(&s, &x1minusx3, &self.p);
        let y3 = FiniteField::subtract(&sx1minusx3, &y1, &self.p);
        (x3, y3)
    }

    fn scalar_mul(&self , j: &Point, k: &BigUint) -> Point {
        //addition/doubling algorithm B = d *A
        // T = A
        //for i in range (0, bits of d - 1)
        // T = 2* T
        // if bit i of d == 1
        // T = T + A
        let mut t = j.clone();
        for i in (0..(k.bits() - 1)).rev() {
            t = self.double(&t);
            if k.bit(i) {
                t = self.add(&t, j);
            }
        }
        t
    }

    pub fn is_on_curve(&self, j: &Point) -> bool {
        // y² = x³ + a*x + b
        match j {
            Point::Coordinate(x, y) => {
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let ax = FiniteField::mul(&self.a, x, &self.p);
                let x3plusax = FiniteField::add(&x3, &ax, &self.p);
                y2 == FiniteField::add(&x3plusax, &self.b, &self.p)
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
        assert!(j < p, "j >= p");
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
        FiniteField::mul(j, &k_inv, p)
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::U32Digits;

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
    fn test_inv_addition_identity() {
        let j = BigUint::from(4u32);
        let p = BigUint::from(51u32);

        let j_inv = FiniteField::inv_addition(&j, &p);

        assert_eq!(j_inv, BigUint::from(47u32));
        assert_eq!(FiniteField::add(&j, &j_inv, &p), BigUint::from(0u32));
    }

    #[test]
    fn test_inv_multiplication_identity() {
        let j = BigUint::from(4u32);
        let p = BigUint::from(11u32);

        let j_inv = FiniteField::inv_multiplication(&j, &p);

        // 4 * 3 mod 11 = 12 mod 11 = 1
        assert_eq!(j_inv, BigUint::from(3u32));
        assert_eq!(FiniteField::mul(&j, &j_inv, &p), BigUint::from(1u32));
    }

    #[test]
    fn test_ec_point_addition() {
        //y² = x³ + 2x + 2 mod 17
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (6,3) + (5,1) = (10,6)
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Coordinate(BigUint::from(10u32), BigUint::from(6u32));

        let res: Point = ec.add(&p1, &p2);
        assert_eq!(res, pr);
    }

    #[test]
    fn test_subtract() {
        let j = BigUint::from(4u32);
        let p = BigUint::from(51u32);

        assert_eq!(FiniteField::subtract(&j, &j, &p), BigUint::from(0u32));
    }

    #[test]
    fn test_divide() {
        let j = BigUint::from(4u32);
        let p = BigUint::from(11u32);

        assert_eq!(FiniteField::divide(&j, &j, &p), BigUint::from(1u32));
    }


    #[test]
    fn test_ec_point_addition_identity() {
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Identity;
        let pr = p1.clone();
        let res = ec.add(&p1, &p2);
        assert_eq!(res, pr);

        let res = ec.add( &p2, &p1);
        assert_eq!(res, pr);
    }


    #[test]
    fn test_ec_point_addition_reflectied_in_x() {
        //y² = x³ + 2x + 2 mod 17
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (5,16) + (5,1) = Point::Identity
        let p1 = Point::Coordinate(BigUint::from(5u32), BigUint::from(16u32));
        let p2 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Identity;

        let res: Point = ec.add(&p1, &p2);
        assert_eq!(res, pr);
    }

    #[test]
    fn test_ec_point_doubling() {
        //y² = x³ + 2x + 2 mod 17
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (5,1) + (5,1) = (6,3)
        let p1 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));

        let res: Point = ec.double(&p1);
        assert_eq!(res, pr);
    }


    #[test]
    fn test_ec_point_doubling_identity() {
        //y² = x³ + 2x + 2 mod 17
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // I + I = I
        let p1 = Point::Identity;
        let pr = Point::Identity;

        let res: Point = ec.double(&p1);
        assert_eq!(res, pr);
    }



    #[test]
    fn test_ec_scalar_multiplication() {
        //y² = x³ + 2x + 2 mod 17 |G| = 19    19 * A = I
        let ec = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (5,1) + (5,1) = (6,3)
        let j = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));

        let pr = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let res: Point = ec.scalar_mul(&j, &BigUint::from(2u32));
        assert_eq!(res, pr);

        // 10 (5,1)  = (7,11)
        let pr = Point::Coordinate(BigUint::from(7u32), BigUint::from(11u32));
        let res: Point = ec.scalar_mul(&j, &BigUint::from(10u32));
        assert_eq!(res, pr);
    }


    #[test]
    fn test_ec_secp256k1() {
        // p = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE FFFFFC2F
        // a = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
        // b = 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000007
        // G = 04 79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798 483ADA77 26A3C465 5DA4FBFC 0E1108A8 FD17B448 A6855419 9C47D08F FB10D4B8
        // n = FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFE BAAEDCE6 AF48A03B BFD25E8C D0364141
        //

        let p = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16
        ).expect("could not convert p");

        let n = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16
        ).expect("could not convert n");

        let gx = BigUint::parse_bytes(
            b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 
            16
        ).expect("could not convert gx");

        let gy = BigUint::parse_bytes(
            b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 
            16
        ).expect("could not convert gy");


        let ec = EllipticCurve {
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p,

        };

        let g = Point::Coordinate((gx), (gy));
        let res = ec.scalar_mul(&g, &n,); // n * G

        assert_eq!(res, Point::Identity);
        

    }
}
