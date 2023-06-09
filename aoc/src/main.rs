use clap::{Parser, CommandFactory};
use std::process::exit;
use std::io::ErrorKind;
use aoc::*;

const YEAR: u16 = 2021;

// all puzzle days. Note that the puzzle number should be the first number in the directory name.
const DAYS: &'static [Day] = &[
    Day{ dir: "day1_sonar_sweep", solve: day1_sonar_sweep::solve },
    Day{ dir: "day2_dive", solve: day2_dive::solve },
    Day{ dir: "day3_binary_diagnostic", solve: day3_binary_diagnostic::solve },
    Day{ dir: "day4_giant_squid", solve: day4_giant_squid::solve },
    Day{ dir: "day5_hydrothermal_venture", solve: day5_hydrothermal_venture::solve },
    Day{ dir: "day6_lanternfish", solve: day6_lanternfish::solve },
    Day{ dir: "day7_treachery_of_whales", solve: day7_treachery_of_whales::solve },
    Day{ dir: "day8_seven_segment", solve: day8_seven_segment::solve },
    Day{ dir: "day9_smoke_basin", solve: day9_smoke_basin::solve },
    Day{ dir: "day10_syntax_scoring", solve: day10_syntax_scoring::solve },
    Day{ dir: "day11_dumbo_octopus", solve: day11_dumbo_octopus::solve },
    Day{ dir: "day12_passage_passing", solve: day12_passage_passing::solve },
    Day{ dir: "day13_transparent_origami", solve: day13_transparent_origami::solve },
    Day{ dir: "day14_extended_polymerization", solve: day14_extended_polymerization::solve },
    Day{ dir: "day15_chiton", solve: day15_chiton::solve },
];

fn main() {
    let args = CliArgs::parse();
    // reject "--all" and explicit puzzle numbers
    if args.all && !args.puzzle.is_empty() {
        let mut cmd = CliArgs::command();
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
        run_puzzles(rootdir, &args, &DAYS, YEAR);
    } else if !args.puzzle.is_empty() {
        run_puzzles(rootdir, &args, &to_days(&args.puzzle, &DAYS), YEAR);
    } else {
        let puzzle = current_puzzle(&DAYS);
        match puzzle {
            Ok(d) => run_puzzles(rootdir, &args, d, YEAR),
            Err(e) if e.kind() == ErrorKind::NotFound => run_puzzles(rootdir, &args, &DAYS[DAYS.len()-1..], YEAR),
            Err(e) => {
                eprintln!("Error searching for puzzle from current dir: {e}");
                exit(1);
            },
        };
    }
}

