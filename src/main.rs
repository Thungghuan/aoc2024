use aoc2024::puzzles::*;
use clap::{Parser, Subcommand};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::io::prelude::*;
use std::{error::Error, fs::File};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day { day: i32 },
}

async fn get_input(day: i32) -> Result<String, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/2024/day/{:?}/input", day);
    let mut cookies = String::new();
    File::open(".cookie")?.read_to_string(&mut cookies)?;

    let mut headers = HeaderMap::new();
    headers.insert("COOKIE", HeaderValue::from_str(&cookies)?);
    let client = Client::builder().default_headers(headers).build()?;

    let resp = client.get(url).send().await?;
    assert_eq!(
        resp.status(),
        200,
        "Error getting puzzle import. Message: {:#?}",
        resp.text().await?
    );

    Ok(resp.text().await?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Day { day } => {
            let data = get_input(day.clone()).await?;

            match day {
                1 => {
                    let puzzle = day1::Day1;
                    puzzle.solve(&data);
                }
                2 => {
                    let puzzle = day2::Day2;
                    puzzle.solve(&data);
                }
                3 => {
                    let puzzle = day3::Day3;
                    puzzle.solve(&data);
                }
                4 => {
                    let puzzle = day4::Day4;
                    puzzle.solve(&data);
                }
                5 => {
                    let puzzle = day5::Day5;
                    puzzle.solve(&data);
                }
                6 => {
                    let puzzle = day6::Day6;
                    puzzle.solve(&data);
                }
                7 => {
                    let puzzle = day7::Day7;
                    puzzle.solve(&data);
                }
                8 => {
                    let puzzle = day8::Day8;
                    puzzle.solve(&data);
                }
                9 => {
                    let puzzle = day9::Day9;
                    puzzle.solve(&data);
                }
                10 => {
                    let puzzle = day10::Day10;
                    puzzle.solve(&data);
                }
                11 => {
                    let puzzle = day11::Day11;
                    puzzle.solve(&data);
                }

                _ => {
                    println!("Puzzle of day {:#?} not found!", day);
                }
            }
        }
    };

    Ok(())
}

