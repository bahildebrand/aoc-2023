mod challenges;
mod day;

use crate::challenges::day1::Day1;
use crate::day::Day;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which day to run
    #[arg(short, long)]
    day: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let days: Vec<Box<dyn Day>> = vec![Box::new(Day1)];

    match args.day {
        Some(input_day) => {
            if input_day > days.len() {
                println!("Day {} not implemented yet", input_day);
            } else if input_day == 0 || input_day > 25 {
                println!("Invalid day number: {}", input_day);
            } else {
                let day = &days[input_day - 1];
                run_day(day, input_day);
            }
        }
        None => {
            for (day_num, day) in days.iter().enumerate() {
                run_day(&day, day_num + 1);
            }
        }
    }
}

fn run_day(day: &Box<dyn Day>, day_number: usize) {
    println!("Day {} part 1:\n{}", day_number, day.part1());
    println!("Day {} part 2:\n{}", day_number, day.part2());
}
