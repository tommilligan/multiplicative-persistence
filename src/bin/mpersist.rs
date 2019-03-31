extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate num_cpus;
extern crate threadpool;

extern crate multiplicative_persistence;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Instant;

use clap::{App, Arg, SubCommand};
use threadpool::ThreadPool;

use multiplicative_persistence::{multiplicative_persistence, SearchResult, SearchRound};

/// Consume a single search round, reporting results to the main thread.
pub fn search_round(tx: Sender<SearchResult>, n: usize) -> () {
    let round_start = Instant::now();
    for result in SearchRound::new(n) {
        tx.send(result).expect("Failed to send SearchResult");
    }
    info!(
        "info: round {} complete in {}ms",
        n,
        round_start.elapsed().as_millis()
    );
}

/// Multithreaded search for integers with higher multiplicative persistence values.
fn search(from_round: usize, num_rounds: usize, n_workers: usize) -> () {
    let pool = ThreadPool::new(n_workers);
    let mut receivers: Vec<Receiver<SearchResult>> = Vec::new();

    for n in from_round..(from_round + num_rounds) {
        let (tx, rx): (Sender<SearchResult>, Receiver<SearchResult>) = channel();
        pool.execute(move || search_round(tx, n));
        receivers.push(rx);
    }

    let mut current_max = 2;
    // Read results from our workers in order
    for rx in receivers {
        loop {
            match rx.recv() {
                // Validate this worker's result with parent state
                Ok(SearchResult {
                    candidate,
                    multiplicative_persistence,
                }) => {
                    if multiplicative_persistence > current_max {
                        current_max += 1;
                        println!("{} {}", multiplicative_persistence, candidate);
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
    env_logger::init();
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
