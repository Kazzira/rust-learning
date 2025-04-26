use num_bigint::{BigInt, RandBigInt, ToBigInt};
use std::mem;

/// Factor out the largest power of 2 from a BigInt.
///
/// # Arguments
/// * `n` - A reference to the BigInt number.
///
/// # Returns
/// * A tuple containing:
///  - The remaining part of the number after factoring out the largest power of 2.
///  - The exponent of the power of 2 that was factored out.
///
///  # Examples
///  ```
///  use num_bigint::{BigInt, ToBigInt};
///  use rsa::math::factor_power_2;
///
///  let n = 8.to_bigint().unwrap();
///  assert_eq!(factor_power_2(&n), (1.to_bigint().unwrap(), 3));
///  ```
///
///  ```
///  use num_bigint::{BigInt, ToBigInt};
///  use rsa::math::factor_power_2;
///
///  let n = 10.to_bigint().unwrap();
///  assert_eq!(factor_power_2(&n), (5.to_bigint().unwrap(), 1));
///  ```
pub fn factor_power_2(n: &BigInt) -> (BigInt, u32) {
    let mut n = n.clone();
    let mut k = 0;
    let zero = 0.to_bigint().unwrap();

    while &n % 2 == zero {
        n /= 2;
        k += 1;
    }

    (n, k)
}

/// Calculate the greatest common divisor (GCD) of two BigInt numbers using the Euclidean algorithm.
///
/// # Arguments
/// * `a` - A reference to the first BigInt number.
/// * `b` - A reference to the second BigInt number.
///
/// # Returns
/// * A BigInt representing the GCD of the two numbers.
///
/// # Examples
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::gcd;
///
/// let a = 5.to_bigint().unwrap();
/// let b = 8.to_bigint().unwrap();
/// assert_eq!(gcd(&a, &b), 1.to_bigint().unwrap());
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::gcd;
///
/// let a = 8.to_bigint().unwrap();
/// let b = 5.to_bigint().unwrap();
/// assert_eq!(gcd(&a, &b), 1.to_bigint().unwrap());
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::gcd;
///
/// let a = 8.to_bigint().unwrap();
/// let b = 4.to_bigint().unwrap();
/// assert_eq!(gcd(&a, &b), 4.to_bigint().unwrap());
/// ```
pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut r0 = a.clone();
    let mut r1 = b.clone();

    if r0 < r1 {
        mem::swap(&mut r0, &mut r1);
    }

    let mut r2 = &r0 % &r1;
    let zero = 0.to_bigint().unwrap();

    while r2 != zero {
        r0 = r1;
        r1 = r2;

        r2 = &r0 % &r1;
    }

    r1
}

/// Performs the reverse Euclidean algorithm to find the modular inverse of `a` modulo `b`.
///
/// # Arguments
/// * `a` - A reference to the BigInt number for which the modular inverse is to be found.
/// * `b` - A reference to the BigInt number modulo which the inverse is calculated.
///
/// # Returns
/// * A BigInt representing the modular inverse of `a` modulo `b`, or 0 if the inverse does not exist.
///
/// # Examples
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::multiplicative_inverse;
///
/// let a = 3.to_bigint().unwrap();
/// let b = 11.to_bigint().unwrap();
/// assert_eq!(multiplicative_inverse(&a, &b), 4.to_bigint().unwrap());
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::multiplicative_inverse;
///
/// let a = 10.to_bigint().unwrap();
/// let b = 17.to_bigint().unwrap();
/// assert_eq!(multiplicative_inverse(&a, &b), 12.to_bigint().unwrap());
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::multiplicative_inverse;
///
/// let a = 5.to_bigint().unwrap();
/// let b = 8.to_bigint().unwrap();
/// assert_eq!(multiplicative_inverse(&a, &b), 5.to_bigint().unwrap());
/// ```
pub fn multiplicative_inverse(a: &BigInt, b: &BigInt) -> BigInt {
    let mut r0 = a.clone();
    let mut r1 = b.clone();

    if r0 < r1 {
        mem::swap(&mut r0, &mut r1);
    }

    let mut t0 = 0.to_bigint().unwrap();
    let mut t1 = 1.to_bigint().unwrap();

    let zero = 0.to_bigint().unwrap();

    while r1 != zero {
        let q = &r0 / &r1;
        let r2 = &r0 - &q * &r1;
        let t2 = &t0 - &q * &t1;

        r0 = r1;
        r1 = r2;
        t0 = t1;
        t1 = t2;
    }

    if t0 < zero {
        t0 += b;
    }

    t0
}

/// Perform the Miller-Rabin primality test.
///
/// # Arguments
/// * `n` - A reference to the BigInt number to be tested for primality.
/// * `b` - The miller base for the Miller-Rabin test.
///
/// # Returns
/// * A boolean indicating whether the number is likely prime.
///
/// # Examples
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::miller_test;
///
/// let n = 7.to_bigint().unwrap();
/// assert!(miller_test(&n, 2));
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::miller_test;
///
/// let n = 10.to_bigint().unwrap();
/// assert!(!miller_test(&n, 2));
/// ```
pub fn miller_test(n: &BigInt, b: u64) -> bool {
    let one = BigInt::from(1);
    let n_minus_one = n - &one;
    let (t, s) = factor_power_2(&n_minus_one);

    let base = BigInt::from(b);

    let mut x = base.modpow(&t, &n);

    if x == one || x == n_minus_one {
        return true;
    }

    let two = BigInt::from(2);

    for counter in 0..s {
        let exp = two.pow(counter) * &t;
        x = base.modpow(&exp, &n);
        if x == n_minus_one {
            return true;
        }
    }

    false
}

/// Check if a number is prime using the Miller-Rabin primality test.
/// The return value may be a false positive if the max_miller_bases is too low.
///
/// # Arguments
/// * `p` - A reference to the BigInt number to be tested for primality.
/// * `max_miller_bases` - The maximum number of Miller bases to use for the test.
///
/// # Returns
/// * A boolean indicating whether the number is likely prime.
///
/// # Examples
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::is_prime;
///
/// let p = 7.to_bigint().unwrap();
/// assert!(is_prime(&p, 5));
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::is_prime;
///
/// let p = 10.to_bigint().unwrap();
/// assert!(!is_prime(&p, 5));
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::is_prime;
///
/// let p = 2.to_bigint().unwrap();
/// assert!(is_prime(&p, 5));
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::is_prime;
///
/// let p = 1.to_bigint().unwrap();
/// assert!(!is_prime(&p, 5));
/// ```
pub fn is_prime(p: &BigInt, max_miller_bases: u64) -> bool {
    let zero = BigInt::from(0);
    let two = BigInt::from(2);

    if *p < two {
        return false;
    }

    if *p == two {
        return true;
    }

    if p % &two == zero {
        return false;
    }

    let mut b = 2;

    while b < max_miller_bases {
        if !miller_test(p, b) {
            return false;
        }

        b += 1;
    }

    true
}

/// Generate a random prime number with the specified number of bits.
///
/// # Arguments
/// * `bits` - The number of bits for the prime number.
///
/// # Returns
/// * A BigInt representing the generated prime number.
///
/// # Examples
/// ```
/// use rsa::math::generate_random_prime;
/// use rsa::math::is_prime;
///
/// let prime = generate_random_prime(128);
/// assert!(is_prime(&prime, 5));
/// ```
pub fn generate_random_prime(bits: u64) -> BigInt {
    let mut rng = rand::thread_rng();
    let mut prime = BigInt::from(0);
    let bases = bits;
    let zero = BigInt::from(0);
    let two = BigInt::from(2);

    while !is_prime(&prime, bases) {
        prime = rng.gen_bigint(bits);

        if prime < zero {
            prime = -prime;
        }

        if &prime % &two == zero {
            prime += 1;
        }
    }

    prime
}

#[test]
fn gcd_of_5_and_8_is_1() {
    let a = 5.to_bigint().unwrap();
    let b = 8.to_bigint().unwrap();

    assert_eq!(gcd(&a, &b), 1.to_bigint().unwrap());
}

#[test]
fn gcd_of_8_and_5_is_1() {
    let a = 8.to_bigint().unwrap();
    let b = 5.to_bigint().unwrap();

    assert_eq!(gcd(&a, &b), 1.to_bigint().unwrap());
}

#[test]
fn gcd_of_8_and_4_is_4() {
    let a = 8.to_bigint().unwrap();
    let b = 4.to_bigint().unwrap();

    assert_eq!(gcd(&a, &b), 4.to_bigint().unwrap());
}

#[test]
fn factor_power_2_of_8_is_3() {
    let n = 8.to_bigint().unwrap();
    assert_eq!(factor_power_2(&n), (1.to_bigint().unwrap(), 3));
}

#[test]
fn factor_power_2_of_10_is_1() {
    let n = 10.to_bigint().unwrap();
    assert_eq!(factor_power_2(&n), (5.to_bigint().unwrap(), 1));
}

#[test]
fn multiplicative_inverse_of_3_and_11_is_4() {
    let a = 3.to_bigint().unwrap();
    let b = 11.to_bigint().unwrap();

    assert_eq!(multiplicative_inverse(&a, &b), 4.to_bigint().unwrap());
}

#[test]
fn generate_random_prime_is_not_negative() {
    let bits = 128;
    let prime = generate_random_prime(bits);
    assert!(prime > 0.to_bigint().unwrap());
}

#[test]
fn multiplicative_inverse_of_10_and_17_is_12() {
    let a = 10.to_bigint().unwrap();
    let b = 17.to_bigint().unwrap();

    assert_eq!(multiplicative_inverse(&a, &b), 12.to_bigint().unwrap());
}

#[test]
fn multiplicative_inverse_of_5_and_8_is_5() {
    let a = 5.to_bigint().unwrap();
    let b = 8.to_bigint().unwrap();

    assert_eq!(multiplicative_inverse(&a, &b), 5.to_bigint().unwrap());
}
