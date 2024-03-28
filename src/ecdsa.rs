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
    pub fn generate_key_pair(&self) -> (BigUint, Point) {
        let priv_key = self.generate_priv_key();
        let pub_key = self.generate_public_key(&priv_key);
        (priv_key, pub_key)
        
    }

    pub fn generate_priv_key(&self) -> BigUint {
        self.generate_random_positive_number_within_range(&self.q_order)

    }

    // (0, max)
    pub fn generate_random_positive_number_within_range(&self, max: &BigUint) -> BigUint {
        todo!();

    }


    pub fn generate_public_key(&self, priv_key: &BigUint) -> BigUint {
        todo!();

    }


    pub fn sign(
        &self,
        hash: BigUint,
        priv_key: BigUint,
        key_random: &BigUint
    ) -> (BigUint, BigUint) {
        todo!();
    }

    pub fn verify (&self, hash: &BigUint, pub_key: Point, signature: &(BigUint, BigUint)) -> bool {

    }
}