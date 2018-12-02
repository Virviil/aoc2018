use std::fs::File;
use std::io::prelude::*;

extern crate clap;

mod checksum;
mod frequency;

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
            },
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
