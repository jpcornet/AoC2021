use std::io::BufReader;
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
    puzzle: Vec<u8>,
}

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
    }
    // // XXX refactor me
    // let mut puzzles: Vec<&Day> = Vec::new();
    // if args.puzzle.len() > 0 {
    //     let mut to_run: HashMap<u8, ()> = HashMap::new();
    //     for p in args.puzzle {
    //         let e = to_run.entry(p);
    //         if let Entry::Occupied(_) = e {
    //             eprintln!("You want to run puzzle {p} twice?");
    //             exit(2);
    //         }
    //         e.or_insert(());
    //     }
    //     for d in DAYS {
    //         let numstart = d.dir.find(char::is_ascii_digit).expect("All directories should include a day number");
    //         let numend = numstart + d.dir[numstart..].find(|d| !d.is_ascii_digit()).or(len(d[numstart..])).unwrap();
    //         let dnum: u8 = d.dir[numstart..numend].parse().expect("Day number should be sane");
    //         if to_run.contains_key(dnum) {
    //             puzzles.push(d);
    //             to_run.remove(dnum);
    //         }
    //         if !to_run.is_empty() {
    //             eprintln!("Puzzles not found: {}", to_run.keys().collect::<Vec<_>>());
    //             exit(2);
    //         }
    //     }
    // } else if !args.all {
    //     // get the current directory, and see if any path is a puzzle directory
    //     let curdir = std::env::current_dir();
    //     if let Ok(cd) = curdir {

    //         cd.components().find(|c| DAYS.iter().find(|d| c == Component::Normal(&d.dir) ))
    //     }
    // }
}

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
            return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Cannot find target directory"));
        }
        // if it's in the seen map, we have seen this dir already and we can abort with a "not found"
        let e = seen.entry(d.to_path_buf());
        if let Entry::Occupied(_) = e {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find target directory"));
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
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Searched all the way to the top, nothing found"))
}
