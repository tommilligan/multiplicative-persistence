extern crate multiplicative_persistence;

use multiplicative_persistence::combinations_wr::CombinationsWithReplacement;
use multiplicative_persistence::{multiplicative_persistence, DIGITS_HEAD, DIGITS_TAIL};

pub fn main() {
    let combinations = CombinationsWithReplacement::new(DIGITS_TAIL.to_vec(), 2);
    for combination in combinations {
        let heads = DIGITS_HEAD.iter();
        for head in heads {
            let mut digits = combination.clone();
            digits.push(*head);
            let input: String = digits.iter().collect();
            let result = multiplicative_persistence(&input);
            println!("{} {}", &input, &result);
        }
    }
}
