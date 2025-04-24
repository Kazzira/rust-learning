pub mod math {
    use num_bigint::{BigInt, ToBigInt};
    use std::mem;

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
    /// use rsa::math::math::gcd;
    ///
    /// let a = 5.to_bigint().unwrap();
    /// let b = 8.to_bigint().unwrap();
    /// assert_eq!(gcd(&a, &b), 1.to_bigint().unwrap());
    /// ```
    ///
    /// ```
    /// use num_bigint::{BigInt, ToBigInt};
    /// use rsa::math::math::gcd;
    ///
    /// let a = 8.to_bigint().unwrap();
    /// let b = 5.to_bigint().unwrap();
    /// assert_eq!(gcd(&a, &b), 1.to_bigint().unwrap());
    /// ```
    ///
    /// ```
    /// use num_bigint::{BigInt, ToBigInt};
    /// use rsa::math::math::gcd;
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
}
