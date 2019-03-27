#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::Num;
use std::sync::mpsc::Sender;
use std::time::Instant;

pub mod combinations_wr;

use combinations_wr::CombinationsWithReplacement;

lazy_static! {
    static ref DIGITS_HEAD: &'static [&'static str; 4] = &["", "2", "3", "4"];
    static ref DIGITS_TAIL: &'static [char; 4] = &['6', '7', '8', '9'];
    static ref BIG_UINT_NINE: BigUint = BigUint::from(9 as usize);
}

/// Multiply digits of an integer together and return the result.
pub fn multiply_digits(a: &BigUint) -> BigUint {
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

#[derive(Debug)]
pub struct SearchMessage {
    pub candidate: String,
    pub mp: usize,
}

pub fn search_round(tx: Sender<SearchMessage>, n: usize) -> () {
    let round_start = Instant::now();

    // Only send messages with potentially higher mp
    let mut current_max = 2;

    // Iterate through integers in ascending order
    let tail_combinations = CombinationsWithReplacement::new(DIGITS_TAIL.to_vec(), n);
    for head in DIGITS_HEAD.iter() {
        let combinations = tail_combinations.clone();
        for combination in combinations {
            let mut candidate: String = head.to_string();
            let tail: String = combination.iter().collect();
            candidate.push_str(&tail);

            // Calculate mp from an interger held as a string
            let result = multiplicative_persistence(&candidate);
            // If we have a potentially better value, report it
            if result > current_max {
                current_max = result;
                tx.send(SearchMessage {
                    candidate,
                    mp: result,
                })
                .expect("Failed to send SearchMessage");
            }
        }
    }

    info!(
        "info: round {} complete in {}ms",
        n,
        round_start.elapsed().as_millis()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test helper to cut down on boilerplate
    fn big(n: usize) -> BigUint {
        BigUint::from(n)
    }

    // Over 64-bit integer
    const TOO_LARGE_INT: &'static str =
        "12346789123467891234678912346789123467891234678912346789123467891234678912346789123467891234678912346789123467891234678912346789";

    #[test]
    fn test_multiplicative_persistence() {
        assert_eq!(multiplicative_persistence("0"), 0);
        assert_eq!(multiplicative_persistence("3"), 0);
        assert_eq!(multiplicative_persistence("24"), 1);
        assert_eq!(multiplicative_persistence("39"), 3);
        assert_eq!(multiplicative_persistence(&TOO_LARGE_INT), 2);
    }

    #[test]
    fn test_multiply_digits() {
        assert_eq!(multiply_digits(&big(0)), big(0));
        assert_eq!(multiply_digits(&big(3)), big(3));
        assert_eq!(multiply_digits(&big(24)), big(8));
        assert_eq!(multiply_digits(&big(12345)), big(120));
    }

}
