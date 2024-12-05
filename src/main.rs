mod questions;
mod solution;
use clap::{Parser, Subcommand};
use questions::q1::Q1;
use solution::Solution;
use std::{ops::RangeInclusive, usize};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Solve a problem
    Solve {
        /// The day's problem to solve
        #[arg(value_parser = date_validator)]
        day: u8,
        /// Part of the problem to solve either 1 or 2
        #[arg(value_parser = question_part_validator, default_value_t = 1)]
        part: u8,
        /// input file for the question, by default it's input/dayN.txt where N is the day's input
        #[arg(short, long)]
        input: Option<String>,
    },
}

const DATE_RANGE: RangeInclusive<usize> = 1..=25;

fn date_validator(s: &str) -> Result<u8, String> {
    let date: usize = s.parse().map_err(|_| format!("`{s}` is not a number"))?;
    if DATE_RANGE.contains(&date) {
        Ok(date as u8)
    } else {
        Err(format!("{} should be between 1 and 25", date))
    }
}

fn question_part_validator(s: &str) -> Result<u8, String> {
    let question_part: u8 = s.parse().map_err(|_| format!("`{s}` is not a number"))?;
    if question_part != 1 || question_part != 2 {
        Err(format!("question_part can only either be 1 or 2"))
    } else {
        Ok(question_part as u8)
    }
}

fn main() {
    // let cli = Cli::parse();
    // match &cli.command {
    //     Commands::Solve { day, part, input } => match day {
    //         1 => match part {
    //             1 => {}
    //             2 => {}
    //             _ => {}
    //         },
    //         2 => match part {
    //             1 => {}
    //             2 => {}
    //             _ => {}
    //         },
    //         3 => match part {
    //             1 => {}
    //             2 => {}
    //             _ => {}
    //         },
    //         _ => {}
    //     },
    // }
    let q1 = Q1 {};
    println!("{}", q1.solve_part_two(None))
}
