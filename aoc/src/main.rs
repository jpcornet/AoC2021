use clap::{CommandFactory, Parser};
use std::process::exit;
use std::io::ErrorKind;
use aoc::*;

/// command line tool to run Advent of Code puzzles and display output and timings
/// 
/// This tool will run the Advent of Code puzzles, by default the latest one or the
/// one given on the command line, or the one in the subdirectory where you are.
/// Will give "raw" output for individual puzzles or present the results in a table,
/// together with timing info.
#[derive(Parser, Debug)]
#[command(author, version = None)]
struct Args {
    /// Run all puzzles
    #[arg(short, long)]
    all: bool,

    /// input file name (default: input.txt)
    #[arg(short, long)]
    input: Option<String>,

    /// which puzzle(s) to run
    puzzle: Vec<u32>,
}

pub const YEAR: u16 = 2021;

// all puzzle days. Note that the puzzle number should be the first number in the directory name.
pub const DAYS: &'static [Day] = &[
    Day{ dir: "day1_sonar_sweep", solve: day1_sonar_sweep::solve },
    Day{ dir: "day2_dive", solve: day2_dive::solve },
];

fn main() {
    let args = Args::parse();
    // reject "--all" and explicit puzzle numbers
    if args.all && args.puzzle.len() > 0 {
        let mut cmd = Args::command();
        cmd.error(clap::error::ErrorKind::ArgumentConflict,
            "Cannot use --all and explicit puzzle numbers.")
            .exit();
    }
    let rootdir = find_root_dir(&DAYS[0].dir);
    if let Err(e) = rootdir {
        eprintln!("Cannot find path to exercises: {:?}", e);
        exit(2);
    }
    let rootdir = rootdir.unwrap();
    // which puzzles to run
    if args.all {
        run_puzzles(rootdir, args.input, &DAYS, YEAR);
    } else if args.puzzle.is_empty() {
        let puzzle = current_puzzle(&DAYS);
        match puzzle {
            Ok(d) => run_puzzles(rootdir, args.input, d, YEAR),
            Err(e) if e.kind() == ErrorKind::NotFound => run_puzzles(rootdir, args.input, &DAYS[DAYS.len()-1..], YEAR),
            Err(e) => {
                eprintln!("Error searching for puzzle from current dir: {e}");
                exit(1);
            },
        };
    } else {
        run_puzzles(rootdir, args.input, &to_days(&args.puzzle, &DAYS), YEAR);
    }
}

