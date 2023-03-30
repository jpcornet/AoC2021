use std::io::{BufReader, ErrorKind};
use std::fs::File;
use exrunner::ExRunner;
use clap::{CommandFactory, Parser};
use std::collections::{HashMap, hash_map::Entry};
use std::path::PathBuf;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::process::exit;

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

#[derive(Clone)]
struct Day {
    dir: &'static str,
    // Need to specify the specific type of BufReader<File> here, because function
    // pointers to generic functions do not exist.
    solve: fn(BufReader<File>, &mut ExRunner),
}

// all puzzle days. Note that the puzzle number should be the first number in the directory name.
const DAYS: &'static [Day] = &[
    Day{ dir: "day1_sonar_sweep", solve: day1_sonar_sweep::solve },
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
    let rootdir = find_root_dir();
    if let Err(e) = rootdir {
        eprintln!("Cannot find path to exercises: {:?}", e);
        exit(2);
    }
    let rootdir = rootdir.unwrap();
    // which puzzles to run
    if args.all {
        run_puzzles(rootdir, args.input, &DAYS);
    } else if args.puzzle.len() == 0 {
        let puzzle = current_puzzle();
        match puzzle {
            Ok(d) => run_puzzles(rootdir, args.input, d),
            Err(e) if e.kind() == ErrorKind::NotFound => run_puzzles(rootdir, args.input, &DAYS[DAYS.len()-1..]),
            Err(e) => {
                eprintln!("Error searching for puzzle from current dir: {e}");
                exit(1);
            },
        };
    } else {
        run_puzzles(rootdir, args.input, &to_days(&args.puzzle));
    }
}

// returns the first number in a string
fn first_number<'a>(input: &'a str) -> &'a str {
    let start_off = input.find(|c: char| c.is_ascii_digit());
    if start_off.is_none() {
        return "";
    }
    let start_off = start_off.unwrap();
    let end_off = input[start_off..].find(|c: char| !c.is_ascii_digit()).unwrap_or(input[start_off..].len());
    &input[start_off..start_off+end_off]
}

// convert list of puzzle numbers to Vec of Day structures.
fn to_days(puzzle: &Vec<u32>) -> Vec<Day> {
    // keep hash of puzzle number and index
    let mut puzzle_pos: HashMap<u32, Option<usize>> = HashMap::new();
    for (index, d) in DAYS.iter().enumerate() {
        let puzzlenum: u32 = first_number(d.dir).parse().expect(&format!("Cannot find puzzle number in {}", d.dir));
        assert!(!puzzle_pos.contains_key(&puzzlenum), "Duplicate puzzle number");
        puzzle_pos.insert(puzzlenum, Some(index));
    }
    let mut result: Vec<Day> = Vec::new();
    for p in puzzle {
        match puzzle_pos.get(p) {
            None => { eprintln!("Puzzle number {p} does not exist"); exit(1); },
            Some(Some(i)) => { result.push(DAYS[*i].clone()); puzzle_pos.insert(*p, None); },
            Some(None) => { eprintln!("Trying to run puzzle {p} twice?"); exit(1); },
        };
    }
    result
}

// Convert current directory to Day ref, or error if not found.
fn current_puzzle() -> std::io::Result<&'static [Day]> {
    // convert directories in DAYS to a hash
    let mut dirpos: HashMap<&str, usize> = HashMap::new();
    for (index, d) in DAYS.into_iter().enumerate() {
        dirpos.insert(d.dir, index);
    }
    let dindex = std::env::current_dir()?.ancestors().find_map(|d| dirpos.get(d.to_string_lossy().as_ref()));
    match dindex {
        Some(i) => Ok(&DAYS[*i..=*i]),
        None => Err(std::io::Error::new(ErrorKind::NotFound, "Current directory is not a puzzle")),
    }
}

// run a list of puzzles
fn run_puzzles(rootdir: PathBuf, input: Option<String>, days: &[Day]) {
    let inputfile  = match input {
        Some(i) => i,
        _ => String::from("input.txt"),
    };
    for d in days {
        let mut fname = rootdir.clone();
        fname.push(d.dir);
        fname.push("input");
        fname.push(&inputfile);
        let fh = File::open(&fname);
        if let Err(e) = fh {
            eprintln!("Error: cannot open file {} for exercise {}: {e}", fname.to_string_lossy(), d.dir);
            continue;
        }
        let er = ExRunner::run(d.dir.to_string(), d.solve, BufReader::new(fh.unwrap()));
        er.print_raw();
    }
}

// libc-specific: get access to uid
#[link(name="c")]
extern "C" {
    fn geteuid() -> u32;
}

// Find the root directory of the puzzles by looking up from the current dir, or from the directory of the binary
fn find_root_dir() -> std::io::Result<PathBuf> {
    // only look at directories that are owned by the current user, so get euid.
    let uid: u32;
    unsafe { uid = geteuid(); }
    // maintain a hash of directories that we looked at
    let mut seen: HashMap<PathBuf, ()> = HashMap::new();
    let root_dir =
        find_in_ancestors(std::env::current_dir()?, &DAYS[0].dir, uid, &mut seen).or_else(|_|
            // search again, from program
            find_in_ancestors(PathBuf::from(std::env::args().next().unwrap()).canonicalize()?, &DAYS[0].dir, uid, &mut seen))?;
    return Ok(root_dir);
}

// find a subdirectory somewhere in the current dir or one of the directories above, only checking directories owned by the given uid.
fn find_in_ancestors(startdir: PathBuf, target: &str, uid: u32, seen: &mut HashMap<PathBuf, ()>) -> std::io::Result<PathBuf> {
    // try_find would make this a bit cleaner, but that's only in nightly at the moment.
    for d in startdir.ancestors() {
        // verify d is a directory and is owned by the right user
        let attr = fs::metadata(d)?;
        // if it's not a directory, skip it.
        if !attr.is_dir() {
            continue;
        }
        // if it has the wrong uid, stop immediately
        if attr.uid() != uid {
            return Err(std::io::Error::new(ErrorKind::PermissionDenied, "Cannot find target directory"));
        }
        // if it's in the seen map, we have seen this dir already and we can abort with a "not found"
        let e = seen.entry(d.to_path_buf());
        if let Entry::Occupied(_) = e {
            return Err(std::io::Error::new(ErrorKind::NotFound, "Cannot find target directory"));
        }
        // try to access the target dir and see if it exists
        let mut targetdir = d.to_path_buf();
        targetdir.push(target);
        let t_attr = fs::metadata(&targetdir);
        if t_attr.is_ok() && t_attr.as_ref().unwrap().is_dir() && t_attr.as_ref().unwrap().uid() == uid {
            // target dir exists, so parent is root dir we want.
            return Ok(d.to_path_buf());
        }
        // mark that we've searched this path, and continue up the tree
        e.or_insert(());
    }
    // if we get here, we didn't find it
    Err(std::io::Error::new(ErrorKind::NotFound, "Searched all the way to the top, nothing found"))
}
