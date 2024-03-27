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

    fn scalar_mul(j: &Point) -> Point {
        //addition/doubling algorithm
        // B = g* A
        todo!()
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
}
