use num_bigint::{BigInt, ToBigInt};
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
///  assert_eq!(factor_power_2(&n), (1.to_bigint().unwrap(), 3.to_bigint().unwrap()));
///  ```
///
///  ```
///  use num_bigint::{BigInt, ToBigInt};
///  use rsa::math::factor_power_2;
///
///  let n = 10.to_bigint().unwrap();
///  assert_eq!(factor_power_2(&n), (5.to_bigint().unwrap(), 1.to_bigint().unwrap()));
///  ```
pub fn factor_power_2(n: &BigInt) -> (BigInt, BigInt) {
    let mut n = n.clone();
    let mut k = 0.to_bigint().unwrap();
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
/// use rsa::math::reverse_euclidean_algorithm;
///
/// let a = 3.to_bigint().unwrap();
/// let b = 11.to_bigint().unwrap();
/// assert_eq!(reverse_euclidean_algorithm(&a, &b), 4.to_bigint().unwrap());
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::reverse_euclidean_algorithm;
///
/// let a = 10.to_bigint().unwrap();
/// let b = 17.to_bigint().unwrap();
/// assert_eq!(reverse_euclidean_algorithm(&a, &b), 12.to_bigint().unwrap());
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::reverse_euclidean_algorithm;
///
/// let a = 5.to_bigint().unwrap();
/// let b = 8.to_bigint().unwrap();
/// assert_eq!(reverse_euclidean_algorithm(&a, &b), 5.to_bigint().unwrap());
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::reverse_euclidean_algorithm;
///
/// let a = 8.to_bigint().unwrap();
/// let b = 4.to_bigint().unwrap();
///
/// assert_eq!(reverse_euclidean_algorithm(&a, &b), 0.to_bigint().unwrap());
/// ```
pub fn reverse_euclidean_algorithm(a: &BigInt, b: &BigInt) -> BigInt {
    let mut r0 = a.clone();
    let mut r1 = b.clone();

    if r0 < r1 {
        mem::swap(&mut r0, &mut r1);
    }

    let mut s0 = 1.to_bigint().unwrap();
    let mut s1 = 0.to_bigint().unwrap();
    let mut t0 = 0.to_bigint().unwrap();
    let mut t1 = 1.to_bigint().unwrap();

    let zero = 0.to_bigint().unwrap();
    let one = 1.to_bigint().unwrap();

    while r1 != zero {
        let q = &r0 / &r1;
        let r2 = &r0 - &q * &r1;

        r0 = r1;
        r1 = r2;

        let s2 = &s0 - &q * &s1;
        s0 = s1;
        s1 = s2;

        let t2 = &t0 - &q * &t1;
        t0 = t1;
        t1 = t2;
    }

    if r0 == one {
        if t0 < zero {
            return b + t0;
        }
        return t0;
    }

    zero
}

/// Perform the Miller-Rabin primality test.
///
/// # Arguments
/// * `n` - A reference to the BigInt number to be tested for primality.
/// * `a` - A reference to the base for the test.
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
/// let a = 2.to_bigint().unwrap();
/// assert!(miller_test(&n, &a));
/// ```
///
/// ```
/// use num_bigint::{BigInt, ToBigInt};
/// use rsa::math::miller_test;
///
/// let n = 9.to_bigint().unwrap();
/// let a = 2.to_bigint().unwrap();
/// assert!(!miller_test(&n, &a));
/// ```
pub fn miller_test(n: &BigInt, a: &BigInt) -> bool {
    let one = 1.to_bigint().unwrap();
    let zero = 0.to_bigint().unwrap();
    let (d, s) = factor_power_2(&(n - &one));

    let s_minus_one = s - one;

    let mut x = a.modpow(&d, &n);

    if x == 1.to_bigint().unwrap() || x == n - 1 {
        return true;
    }

    let mut counter = zero.clone();

    while counter < s_minus_one {
        x = x.modpow(&2.to_bigint().unwrap(), &n);
        if x == n - 1 {
            return true;
        }
        counter += 1;
    }

    false
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
