extern crate multiplicative_persistence;
// use std::time::Instant;

use multiplicative_persistence::combinations_wr::CombinationsWithReplacement;
use multiplicative_persistence::{multiplicative_persistence, DIGITS_HEAD, DIGITS_TAIL};

pub fn main() {
    let start_at = 0;
    let num_rounds = 20;

    let mut current_max = 2;
    for n in start_at..(start_at + num_rounds) {
        //let round_start = Instant::now();
        let tail_combinations = CombinationsWithReplacement::new(DIGITS_TAIL.to_vec(), n);
        for head in DIGITS_HEAD.iter() {
            let combinations = tail_combinations.clone();
            for combination in combinations {
                let mut input: String = head.to_string();
                let tail: String = combination.iter().collect();
                input.push_str(&tail);
                let result = multiplicative_persistence(&input);
                if result > current_max {
                    current_max = result;
                    println!("{} {}", &result, &input);
                }
            }
        }
        //println!(
        //"Round {} completed: {}ms",
        //n,
        //round_start.elapsed().as_millis()
        //);
    }
}
