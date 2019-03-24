#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;

use itertools::Itertools;
use std::iter;

use num_bigint::BigUint;
use num_traits::Num;

lazy_static! {
    static ref DIGITS_HEAD: &'static [char; 3] = &['2', '3', '4'];
    static ref DIGITS_TAIL: &'static [char; 4] = &['6', '7', '8', '9'];
    static ref BIG_UINT_NINE: BigUint = BigUint::from(9 as usize);
}

pub fn multiply_digits(a: &BigUint) -> BigUint {
    a.to_str_radix(10)
        .chars()
        .map(|c| BigUint::from(c.to_digit(10).expect("Could not convert char to digit.")))
        .product()
}

pub fn multiplicative_persistence(int_as_str: &str) -> usize {
    let mut current_int: BigUint =
        Num::from_str_radix(int_as_str, 10).expect("Could not convert string to BigUint");
    let mut counter: usize = 0;

    while current_int > *BIG_UINT_NINE {
        current_int = multiply_digits(&current_int);
        counter += 1;
        println!("{:?}", current_int);
    }
    counter
}

pub fn generate_inputs_n(n: usize) -> impl Iterator<Item = String> {
    let heads = DIGITS_HEAD.iter();
    let tails: Vec<Vec<&'static char>> = DIGITS_TAIL.iter().combinations(n - 1).collect();
    iproduct!(tails, heads).map(|(mut tail, head)| {
        tail.push(head);
        tail.into_iter().collect()
    })
}

//for (i, j, k) in iproduct!(0..4, 0..4, 0..4) {
//// ..
pub fn main() {
    let combinations: Vec<Vec<&char>> = DIGITS_HEAD.iter().combinations(2).collect();
    for combination in combinations {
        println!("{:?}", &combination);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_multiplicative_persistence() {
        assert_eq!(multiplicative_persistence("0"), 0);
        assert_eq!(multiplicative_persistence("3"), 0);
        assert_eq!(multiplicative_persistence("24"), 1);
        assert_eq!(multiplicative_persistence("28"), 2);
    }

    #[test]
    fn test_generate_inputs_n() {
        assert_eq!(
            generate_inputs_n(1).collect::<Vec<String>>(),
            vec!["2", "3", "4"]
        );
        assert_eq!(
            generate_inputs_n(2).collect::<Vec<String>>(),
            vec!["62", "63", "64", "72", "73", "74", "82", "83", "84", "92", "93", "94"]
        );
        assert_eq!(
            generate_inputs_n(3).collect::<Vec<String>>(),
            vec!["62", "63", "64", "72", "73", "74", "82", "83", "84", "92", "93", "94"]
        );
    }

    #[test]
    fn test_multiply_digits() {
        assert_eq!(
            multiply_digits(&BigUint::from(0 as usize)),
            BigUint::from(0 as usize)
        );
        assert_eq!(
            multiply_digits(&BigUint::from(3 as usize)),
            BigUint::from(3 as usize)
        );
        assert_eq!(
            multiply_digits(&BigUint::from(24 as usize)),
            BigUint::from(8 as usize)
        );
    }

}
