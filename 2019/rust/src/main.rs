extern crate clap;

mod one;
mod two;
mod three;

use clap::{
    Arg,
    App,
    SubCommand,
};

fn main() {
    let matches = App::new("aoc2019")
        .about("Advent of Code 2019 Solutions")
        .author("Elijah Voigt")
        .subcommand(SubCommand::with_name("one")
            .about("First day")
            .arg(Arg::with_name("INPUT")
                .help("Input file")
                .required(true)
                .index(1)
            )
        )
        .subcommand(SubCommand::with_name("two")
            .about("Second day")
            .arg(Arg::with_name("INPUT")
                .help("Input string")
                .required(true)
                .index(1)
            )
        )
        .subcommand(SubCommand::with_name("three")
            .about("Third day")
            .arg(Arg::with_name("INPUT A")
                .help("Input string A")
                .required(true)
                .index(1)
            )
            .arg(Arg::with_name("INPUT B")
                .help("Input string B")
                .required(true)
                .index(2)
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("one") {
        let input = matches.value_of("INPUT").unwrap();
        println!("{}", one::solution(input))
    }
    if let Some(matches) = matches.subcommand_matches("two") {
        let input = matches.value_of("INPUT").unwrap();
        println!("{}", two::solution(input))
    }
    if let Some(matches) = matches.subcommand_matches("three") {
        let a = matches.value_of("INPUT A").unwrap();
        let b = matches.value_of("INPUT B").unwrap();
        println!("{}", three::solution(a, b))
    }
}
