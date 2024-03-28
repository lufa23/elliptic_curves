use num_bigint::BigUint;
use ec_generic::{EllipticCurve, FiniteField, Point};
use num_bigint::{BigInt, RandBigInt};
use rand::{self, Rng};
use sha256::{digest,try_digest};


    struct ECDSA {
        elliptic_curve: EllipticCurve,
        a_gen: Point,
        q_order: BigUint,
    }
impl ECDSA {
    
    //generates: d, B where B = d * A
    pub fn generate_key_pair(&self) -> (BigUint, BigUint) {
        let priv_key = self.generate_priv_key();
        let pub_key = self.generate_public_key(&priv_key);
        (priv_key, pub_key)
        
    }

    pub fn generate_priv_key(&self) -> BigUint {
        self.generate_random_positive_number_within_range(&self.q_order)

    }

    // (0, max)
    pub fn generate_random_positive_number_within_range(&self, max: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_range(&BigUint::from(1u32), &max)

    }


    pub fn generate_public_key(&self, priv_key: &BigUint) -> BigUint {
        todo!();

    }

    ///
    /// r = K * a -> take 'r =x ' component
    /// s = (hash(message) + d *r) *k ^(-1) mod q
    ///
    pub fn sign(
        &self,
        hash: &BigUint,
        priv_key: BigUint,
        key_random: &BigUint
    ) -> (BigUint, BigUint) {

        assert!(hash < &self.q_order,
            "Hash is bigger that the order of the group"
        );

        assert!(priv_key < self.q_order,
            "Private Key is bigger that the order of the group"
        );

        assert!(*key_random < self.q_order,
            "Random Number is bigger that the order of the group"
        );

        
            let r_point = self.elliptic_curve.scalar_mul(&self.a_gen, key_random)
    .expect("Scalar multiplication failed");

            if let Point::Coor(r, _) = r_point {
            let s = FiniteField::mult(&r, &priv_key, &self.q_order)
    .expect("Multiplication failed");
let s = match FiniteField::mult(&r, &priv_key, &self.q_order) {
    Ok(value) => value,
    Err(e) => panic!("An error occurred: {:?}", e),
};

            let k_inv = FiniteField::inv_mult_prime(key_random, &self.q_order)
    .expect("Inverse multiplication failed");
            return (r, s);

        }
        panic!("The random point R should not be the identity.")
    }

    pub fn verify (&self, hash: &BigUint, pub_key: Point, signature: &(BigUint, BigUint)) -> bool {
        todo!()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let elliptic_curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(7u32),
        };

        let q_order = BigUint::from(19u32);
        let a_gen = Point::Coor(BigUint::from(5u32), BigUint::from(5u32));
        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa.generate_public_key(&priv_key);

        let hash = BigUint::from(10u32);

        let k_random = BigUint::from(18u32);

        let signature = ecdsa.sign(&hash, priv_key, &k_random);

        println!("{:?}", signature);
    }
}