extern crate clap;

extern crate multiplicative_persistence;

use std::time::Instant;

use clap::{App, Arg, SubCommand};

use multiplicative_persistence::combinations_wr::CombinationsWithReplacement;
use multiplicative_persistence::{multiplicative_persistence, DIGITS_HEAD, DIGITS_TAIL};

fn search(from_round: usize, num_rounds: usize) -> () {
    let mut current_max = 2;
    for n in from_round..(from_round + num_rounds) {
        let round_start = Instant::now();
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
        eprintln!(
            "Round {} completed: {}ms",
            n,
            round_start.elapsed().as_millis()
        );
    }
}

pub fn main() {
    let mut app = App::new("mpersist")
        .about("Find multiplicative persistence values")
        .subcommand(
            SubCommand::with_name("search")
                .about("Search for the lowest integer with each multiplicative persistence value")
                .arg(
                    Arg::with_name("from_round")
                        .help("The search round to start from (length of integer in digits)")
                        .takes_value(true)
                        .short("f")
                        .long("from-round"),
                )
                .arg(
                    Arg::with_name("rounds")
                        .help("The number of search rounds to perform")
                        .takes_value(true)
                        .short("n")
                        .long("rounds"),
                ),
        )
        .subcommand(
            SubCommand::with_name("for")
                .about("Get multiplicative persistence for a positive integer")
                .arg(
                    Arg::with_name("integer")
                        .help("The integer to process")
                        .index(1)
                        .required(true),
                ),
        );
    let matches = app.clone().get_matches();

    if let (subcommand_name, Some(subcommand_matches)) = matches.subcommand() {
        match subcommand_name {
            "for" => {
                let input_integer: &str = subcommand_matches.value_of("integer").unwrap();
                println!("{}", multiplicative_persistence(input_integer));
            }
            "search" => {
                let from_round: usize = subcommand_matches
                    .value_of("from_round")
                    .unwrap_or("0")
                    .parse()
                    .expect("Invalid integer for from_round");
                let num_rounds: usize = subcommand_matches
                    .value_of("rounds")
                    .unwrap_or("15")
                    .parse()
                    .expect("Invalid integer from rounds");
                search(from_round, num_rounds)
            }
            _ => eprintln!("Invalid subcommand."),
        }
    } else {
        app.print_help().expect("Could not print help.");
    }
}
