#[macro_use]
extern crate lazy_static;
extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::Num;

pub mod combinations_wr;
use combinations_wr::CombinationsWithReplacement;

static DIGITS_HEAD: &'static [&'static str; 4] = &["", "2", "3", "4"];
static DIGITS_TAIL: &'static [char; 4] = &['6', '7', '8', '9'];

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

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    pub candidate: String,
    pub multiplicative_persistence: usize,
}

/// An iterator for all search candidates in round n. These follow the pattern
/// of one optional head digit, followed by n optional tail digits.
///
/// - round 1: 6, 7, ..., 9, 26, 27, ..., 49
/// - round 2: 66, 67, ..., 99, 266, 267, ..., 499
#[derive(Debug, Clone)]
struct Candidates<'a> {
    // Cloneable tails iterator, used to restart tails multiple times
    fresh_tails: CombinationsWithReplacement<char>,
    // Heads that we need to iterate over once
    heads: std::slice::Iter<'a, &'a str>,
    current_head: &'a str,
    // The current set of tails we are iterating through
    tails: CombinationsWithReplacement<char>,
}

impl<'a> Candidates<'a> {
    pub fn new(n: usize) -> Candidates<'a> {
        let mut heads = DIGITS_HEAD.iter();
        let fresh_tails = CombinationsWithReplacement::new(DIGITS_TAIL.to_vec(), n);
        Candidates {
            current_head: heads.next().expect("Heads had no items"),
            heads,
            tails: fresh_tails.clone(),

            fresh_tails,
        }
    }
}

impl<'a> Iterator for Candidates<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            match self.tails.next() {
                // If we have another tail combination, combine with head and return
                Some(tail_combination) => {
                    let mut candidate: String = self.current_head.to_string();
                    let tail: String = tail_combination.iter().collect();
                    candidate.push_str(&tail);
                    return Some(candidate);
                }
                // If we've exhausted our set of tail combinations
                // Go to the next head and start tail combinations again
                None => {
                    match self.heads.next() {
                        Some(new_head) => self.current_head = new_head,
                        // If we have no more heads, we're done
                        None => return None,
                    };
                    self.tails = self.fresh_tails.clone();
                }
            }
        }
    }
}

pub struct SearchRound<'a> {
    candidates: Candidates<'a>,
    current_max: usize,
}

impl<'a> SearchRound<'a> {
    pub fn new(n: usize) -> SearchRound<'a> {
        SearchRound {
            candidates: Candidates::new(n),
            current_max: 2,
        }
    }
}

// Only send messages with potentially higher mp
impl<'a> Iterator for SearchRound<'a> {
    type Item = SearchResult;

    fn next(&mut self) -> Option<SearchResult> {
        loop {
            match self.candidates.next() {
                Some(candidate) => {
                    let result = multiplicative_persistence(&candidate);
                    // If we have a potentially better value, report it
                    if result > self.current_max {
                        self.current_max = result;
                        return Some(SearchResult {
                            candidate,
                            multiplicative_persistence: result,
                        });
                    }
                }
                None => return None,
            }
        }
    }
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

    #[test]
    fn test_candidates() {
        let candidates: Vec<String> = Candidates::new(1).collect();
        let expected: Vec<String> = vec![
            "6", "7", "8", "9", "26", "27", "28", "29", "36", "37", "38", "39", "46", "47", "48",
            "49",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(candidates, expected);
    }

    #[test]
    fn test_search_round() {
        let results: Vec<SearchResult> = SearchRound::new(1).collect();
        let expected = vec![SearchResult {
            candidate: "39".to_owned(),
            multiplicative_persistence: 3,
        }];
        assert_eq!(results, expected);
    }
}
