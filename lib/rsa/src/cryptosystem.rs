use crate::math;
use num_bigint::{BigInt, RandBigInt};

pub struct RSAKey {
    pub n: BigInt,
    pub e: BigInt,
    pub d: BigInt,
}

impl RSAKey {
    pub fn new(n: BigInt, e: BigInt, d: BigInt) -> Self {
        RSAKey { n, e, d }
    }

    pub fn encrypt(&self, message: &BigInt) -> BigInt {
        message.modpow(&self.e, &self.n)
    }

    pub fn decrypt(&self, ciphertext: &BigInt) -> BigInt {
        ciphertext.modpow(&self.d, &self.n)
    }

    pub fn sign(&self, message: &BigInt) -> BigInt {
        message.modpow(&self.d, &self.n)
    }

    pub fn verify(&self, signature: &BigInt) -> BigInt {
        signature.modpow(&self.e, &self.n)
    }

    /// Generate a new RSA keypair using two prime numbers p and q.
    ///
    /// # Arguments
    /// * `p` - A prime number.
    /// * `q` - A prime number.
    ///
    /// # Returns
    /// * A new RSAKey instance containing the generated keypair.
    pub fn generate_keypair(p: &BigInt, q: &BigInt) -> Self {
        let n = p * q;
        let phi_n = (p - 1) * (q - 1);
        let e = rsa_make_e(p, q);
        let d = math::multiplicative_inverse(&e, &phi_n);

        RSAKey::new(n, e, d)
    }

    pub fn generate_random_key(bits: u64) -> Self {
        let bits = bits / 2;
        let p = math::generate_random_prime(bits);
        let q = math::generate_random_prime(bits);
        RSAKey::generate_keypair(&p, &q)
    }
}

/// Generate a random number e such that 1 < e < phi(n) and gcd(e, phi(n)) = 1
/// where n = p * q and phi(n) = (p - 1) * (q - 1).
///
/// # Arguments
/// * `p` - A prime number.
/// * `q` - A prime number.
///
/// # Returns
/// * A BigInt representing the generated number e.
///
/// # Examples
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::cryptosystem::rsa_make_e;
/// use rsa::math;
///
/// let p = 61.to_bigint().unwrap();
/// let q = 53.to_bigint().unwrap();
/// let e = rsa_make_e(&p, &q);
/// assert!(e > BigInt::from(1));
/// assert!(e < (p - 1) * (q - 1));
/// ```
pub fn rsa_make_e(p: &BigInt, q: &BigInt) -> BigInt {
    let phi_n = (p - 1) * (q - 1);
    let mut rng = rand::thread_rng();
    let mut e: BigInt;

    while {
        e = rng.gen_bigint_range(&BigInt::from(2), &phi_n);
        math::gcd(&e, &phi_n) != BigInt::from(1)
    } {}

    e
}

#[test]
fn rsa_make_e_should_generate_a_valid_e() {
    let p = BigInt::from(61);
    let q = BigInt::from(53);

    let e = rsa_make_e(&p, &q);

    assert!(e > BigInt::from(1));
    assert!(e < (p - 1) * (q - 1));
}

#[test]
fn rsa_test_key_with_p_7_and_q_5() {
    let p = BigInt::from(5);
    let q = BigInt::from(7);

    let key = RSAKey::generate_keypair(&p, &q);

    let message = BigInt::from(15);
    let ciphertext = key.encrypt(&message);
    let decrypted_message = key.decrypt(&ciphertext);

    assert_eq!(message, decrypted_message);

    // 7 and 5 are not good prime numbers for RSA, because the
    // plaintext is the same as the ciphertext.
    assert_eq!(message, ciphertext);
}

#[test]
fn test_rsa_key_with_128_bits() {
    let bits = 128;
    let key = RSAKey::generate_random_key(bits);

    let message = BigInt::from(153252);
    let ciphertext = key.encrypt(&message);
    let decrypted_message = key.decrypt(&ciphertext);

    assert_eq!(message, decrypted_message);
}

#[test]
fn test_rsa_key_with_256_bits() {
    let bits = 256;
    let key = RSAKey::generate_random_key(bits);

    let message = BigInt::from(153252);
    let ciphertext = key.encrypt(&message);
    let decrypted_message = key.decrypt(&ciphertext);

    assert_eq!(message, decrypted_message);
}
