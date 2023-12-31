mod challenges;
mod day;
mod input;

use crate::challenges::{Day1, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9};
use crate::day::Day;
use crate::input::fetch_input;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(arg_required_else_help = false)]
    Day { day: Option<usize> },
    #[command(arg_required_else_help = true)]
    FetchInput { day: usize },
}

fn main() {
    let args = Args::parse();
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
        Box::new(Day4),
        Box::new(Day5),
        Box::new(Day6),
        Box::new(Day7),
        Box::new(Day8),
        Box::new(Day9),
    ];

    match args.command {
        Commands::Day { day } => day_command(day, days),
        Commands::FetchInput { day } => fetch_input_command(day, days.len()),
    }
}

fn day_command(day: Option<usize>, days: Vec<Box<dyn Day>>) {
    match day {
        Some(input_day) => {
            if validate_day(input_day, days.len()) {
                let day = &days[input_day - 1];
                run_day(day.as_ref(), input_day);
            }
        }
        None => {
            for (day_num, day) in days.iter().enumerate() {
                run_day(day.as_ref(), day_num + 1);
            }
        }
    }
}

fn run_day(day: &dyn Day, day_number: usize) {
    let input = input::get_input(day_number);
    println!("Day {} part 1: {}", day_number, day.part1(&input));
    println!("Day {} part 2: {}", day_number, day.part2(&input));
}

fn fetch_input_command(day: usize, days_implemented: usize) {
    if validate_day(day, days_implemented) {
        fetch_input(day);
    }
}

fn validate_day(day: usize, days_implemented: usize) -> bool {
    if day == 0 || day > 25 {
        println!("Invalid day number: {}", day);
    } else if day > days_implemented {
        println!("Day {} not implemented yet", day);
    } else {
        return true;
    }

    false
}
