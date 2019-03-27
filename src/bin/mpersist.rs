extern crate clap;
extern crate num_cpus;
extern crate threadpool;

extern crate multiplicative_persistence;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;

use clap::{App, Arg, SubCommand};
use threadpool::ThreadPool;

use multiplicative_persistence::combinations_wr::CombinationsWithReplacement;
use multiplicative_persistence::{multiplicative_persistence, DIGITS_HEAD, DIGITS_TAIL};

#[derive(Debug)]
struct SearchMessage {
    candidate: String,
    mp: usize,
}

fn search_round(tx: Sender<SearchMessage>, n: usize) -> () {
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

    eprintln!(
        "info: round {} complete in {}ms",
        n,
        round_start.elapsed().as_millis()
    );
}

/// Multithreaded search for integers with higher multiplicative persistence values.
fn search(from_round: usize, num_rounds: usize, n_workers: usize) -> () {
    let pool = ThreadPool::new(n_workers);
    let mut receivers: Vec<Receiver<SearchMessage>> = Vec::new();

    for n in from_round..(from_round + num_rounds) {
        let (tx, rx): (Sender<SearchMessage>, Receiver<SearchMessage>) = channel();
        pool.execute(move || search_round(tx, n));
        receivers.push(rx);
    }

    let mut current_max = 2;
    // Read results from our workers in order
    for rx in receivers {
        loop {
            match rx.recv() {
                // Validate this worker's result with parent state
                Ok(SearchMessage { candidate, mp }) => {
                    if mp > current_max {
                        current_max += 1;
                        println!("{} {}", mp, candidate);
                    }
                }
                // As soon as we get something other than a result
                // this worker is complete, move on
                _ => break,
            }
        }
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
                        .help("Search round to start from (length of integer in digits)")
                        .takes_value(true)
                        .short("f")
                        .long("from-round"),
                )
                .arg(
                    Arg::with_name("rounds")
                        .help("Number of search rounds to perform")
                        .takes_value(true)
                        .short("n")
                        .long("rounds"),
                )
                .arg(
                    Arg::with_name("threads")
                        .help("Number of threads to use. Default or 0 uses all cores.")
                        .takes_value(true)
                        .short("t")
                        .long("threads"),
                ),
        )
        .subcommand(
            SubCommand::with_name("for")
                .about("Get multiplicative persistence for a positive integer")
                .arg(
                    Arg::with_name("candidate")
                        .help("The integer to process")
                        .index(1)
                        .required(true),
                ),
        );
    let matches = app.clone().get_matches();

    if let (subcommand_name, Some(subcommand_matches)) = matches.subcommand() {
        match subcommand_name {
            "for" => {
                let candidate: &str = subcommand_matches.value_of("candidate").unwrap();
                println!("{}", multiplicative_persistence(candidate));
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
                    .expect("Invalid integer for rounds");
                let mut threads: usize = subcommand_matches
                    .value_of("threads")
                    .unwrap_or("0")
                    .parse()
                    .expect("Invalid integer for threads");

                let available_cpus = num_cpus::get();
                if threads == 0 {
                    threads = available_cpus;
                } else if threads > num_cpus::get() {
                    panic!("Invalid number of threads given.")
                }

                search(from_round, num_rounds, threads)
            }
            _ => eprintln!("Invalid subcommand."),
        }
    } else {
        app.print_help().expect("Could not print help.");
    }
}
