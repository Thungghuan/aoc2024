use aoc2024::puzzles::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day1,
    Day2,
}

fn main() {
    let cli = Cli::parse();

    let puzzle: Box<dyn Puzzle<Input = String, Output = String>> = match &cli.command {
        Commands::Day1 => Box::new(day1::Day1),
        Commands::Day2 => Box::new(day2::Day2),
    };

    puzzle.test()
}
