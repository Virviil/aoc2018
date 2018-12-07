#[macro_use]
extern crate lazy_static;
use std::fs::File;
use std::io::prelude::*;

extern crate chrono;
extern crate clap;
extern crate regex;

mod area;
mod checksum;
mod fabric;
mod frequency;
mod guards;
mod polymer;
mod steps;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Advent of Code")
        .version("1.0")
        .author("Dmitry Rubinsten <rubinsteindb@gmail.com>")
        .about("Solution for Advent of Code 2018 tasks")
        .arg(
            Arg::with_name("input")
                .long("from")
                .short("f")
                .value_name("INPUT FILE")
                .help("input file")
                .required(true)
                .index(1),
        ).arg(
            Arg::with_name("output")
                .long("to")
                .short("t")
                .value_name("OUTPUT FILE")
                .help("output file")
                .index(2),
        ).subcommand(
            SubCommand::with_name("frequency")
                .about("Solution for 1 day's puzzles with frequencies")
                .version("1.0")
                .alias("d1")
                .subcommand(
                    SubCommand::with_name("count")
                        .alias("p1")
                        .about("Count frequencies"),
                ).subcommand(
                    SubCommand::with_name("repeated")
                        .alias("p2")
                        .about("Repeated frequency"),
                ),
        ).subcommand(
            SubCommand::with_name("checksum")
                .about("Solution for 2 day's puzzles with checksums")
                .version("1.0")
                .alias("d2")
                .subcommand(
                    SubCommand::with_name("repeated")
                        .alias("p1")
                        .about("Checks 2 and 3 repeated checksums"),
                ).subcommand(
                    SubCommand::with_name("intersection")
                        .alias("p2")
                        .about("Package intersection"),
                ),
        ).subcommand(
            SubCommand::with_name("fabric")
                .about("Solution for 3 day's puzzles with fabric")
                .version("1.0")
                .alias("d3")
                .subcommand(
                    SubCommand::with_name("overlap")
                        .alias("p1")
                        .about("Count overlap"),
                ).subcommand(
                    SubCommand::with_name("unoverlaped")
                        .alias("p2")
                        .about("Uniq claim"),
                ),
        ).subcommand(
            SubCommand::with_name("guards")
                .about("Solution for 4 day's puzzles with fabric")
                .version("1.0")
                .alias("d4")
                .subcommand(
                    SubCommand::with_name("longest")
                        .alias("p1")
                        .about("Longest multiday streak for particular minute"),
                ).subcommand(
                    SubCommand::with_name("frequency")
                        .alias("p2")
                        .about("Uniq claim"),
                ),
        ).subcommand(
            SubCommand::with_name("polymer")
                .about("Solution for 5 day's puzzles with fabric")
                .version("1.0")
                .alias("d5")
                .subcommand(
                    SubCommand::with_name("triggered")
                        .alias("p1")
                        .about("After triggering"),
                ).subcommand(
                    SubCommand::with_name("best_strand")
                        .alias("p2")
                        .about("Best strand to be removed"),
                ),
        ).subcommand(
            SubCommand::with_name("area")
                .about("Solution for 6 day's puzzles with fabric")
                .version("1.0")
                .alias("d6")
                .subcommand(
                    SubCommand::with_name("largest")
                        .alias("p1")
                        .about("Smallest area"),
                ).subcommand(
                    SubCommand::with_name("region")
                        .alias("p2")
                        .about("Safe are size"),
                ),
        ).subcommand(
            SubCommand::with_name("steps")
                .about("Solution for 7 day's puzzles with fabric")
                .version("1.0")
                .alias("d7")
                .subcommand(
                    SubCommand::with_name("sequence")
                        .alias("p1")
                        .about("Smallest area"),
                ).subcommand(
                    SubCommand::with_name("parallel")
                        .alias("p2")
                        .about("Safe are size"),
                ),
        ).get_matches();

    let input_file = matches.value_of("input").unwrap();
    let _output = matches.value_of("output");

    match matches.subcommand() {
        ("frequency", Some(matches)) => match matches.subcommand_name() {
            Some("count") => {
                let result = frequency::calculate(read_file(input_file));
                println!("{}", result);
            }
            Some("repeated") => {
                let result = frequency::duplication(read_file(input_file));
                println!("{}", result);
            }
            _ => println!("No subcommands were used for command frequency! Try to use help"),
        },
        ("checksum", Some(matches)) => match matches.subcommand_name() {
            Some("repeated") => {
                let result = checksum::checksums(read_file(input_file));
                println!("{}", result);
            }
            Some("intersection") => {
                let result = checksum::similar_packages(read_file(input_file));
                println!("{}", result);
            }
            _ => println!("No subcommands were used for command frequency! Try to use help"),
        },
        ("fabric", Some(matches)) => match matches.subcommand_name() {
            Some("overlap") => {
                let result = fabric::overlap(read_file(input_file));
                println!("{}", result);
            }
            Some("unoverlaped") => {
                let result = fabric::unoverlaped(read_file(input_file));
                println!("{}", result);
            }
            _ => println!("No subcommands were used for command frequency! Try to use help"),
        },
        ("guards", Some(matches)) => match matches.subcommand_name() {
            Some("longest") => {
                let result = guards::longest(read_file(input_file));
                println!("{}", result);
            }
            Some("frequency") => {
                let result = guards::most_frequent(read_file(input_file));
                println!("{}", result);
            }
            _ => println!("No subcommands were used for command frequency! Try to use help"),
        },
        ("polymer", Some(matches)) => match matches.subcommand_name() {
            Some("triggered") => {
                let result = polymer::triggered(read_file(input_file));
                println!("{}", result);
            }
            Some("best_strand") => {
                let result = polymer::best_strand(read_file(input_file));
                println!("{}", result);
            }
            _ => println!("No subcommands were used for command frequency! Try to use help"),
        },
        ("area", Some(matches)) => match matches.subcommand_name() {
            Some("largest") => {
                let result = area::largest(read_file(input_file));
                println!("{}", result);
            }
            Some("region") => {
                let result = area::region(read_file(input_file));
                println!("{}", result);
            }
            _ => println!("No subcommands were used for command frequency! Try to use help"),
        },
        ("steps", Some(matches)) => match matches.subcommand_name() {
            Some("sequence") => {
                let result = steps::sequence(read_file(input_file));
                println!("{}", result);
            }
            Some("parallel") => {
                let result = steps::parallel(read_file(input_file), 60, 5);
                println!("{}", result);
            }
            _ => println!("No subcommands were used for command frequency! Try to use help"),
        },
        ("", None) => println!("No subcommands were used! Try to use help"),
        _ => unreachable!(),
    }
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found!");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong while reading the file!");
    contents
}
