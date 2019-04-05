extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::Num;

lazy_static! {
    static ref BIG_UINT_NINE: BigUint = BigUint::from(9 as usize);
}

/// Multiply digits of an integer together and return the result.
fn multiply_digits(a: &BigUint) -> BigUint {
    a.to_str_radix(10)
        .chars()
        .map(|c| BigUint::from(c.to_digit(10).expect("Could not convert char to digit.")))
        .product()
}

/// Return the multiplicative persistence of a positive integer given as a string.
pub fn multiplicative_persistence(candidate: &str) -> usize {
    let mut derived_int: BigUint =
        Num::from_str_radix(candidate, 10).expect("Could not convert candidate to BigUint");

    let mut counter: usize = 0;
    while derived_int > *BIG_UINT_NINE {
        derived_int = multiply_digits(&derived_int);
        counter += 1;
    }
    counter
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test helper to cut down on boilerplate
    fn big(n: usize) -> BigUint {
        BigUint::from(n)
    }

    #[test]
    fn test_multiply_digits() {
        assert_eq!(multiply_digits(&big(0)), big(0));
        assert_eq!(multiply_digits(&big(3)), big(3));
        assert_eq!(multiply_digits(&big(24)), big(8));
        assert_eq!(multiply_digits(&big(12345)), big(120));
    }
}
