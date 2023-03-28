use std::io::BufReader;
use std::fs::File;
use exrunner::ExRunner;
use clap::{CommandFactory, Parser};
use std::collections::{HashMap, hash_map::Entry};
use std::path::PathBuf;
use std::fs;
use std::os::unix::fs::MetadataExt;

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

    /// input file name
    #[arg(short, long, default_value_t=String::from("input.txt"))]
    input: String,

    /// which puzzle(s) to run
    puzzle: Vec<u8>,
}

struct Day {
    dir: &'static str,
    // Need to specify the specific type of BufReader<File> here, because function
    // pointers to generic functions do not exist.
    solve: fn(BufReader<File>, &mut ExRunner),
}

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
    if let std::io::Result::Err(e) = rootdir {
        eprintln!("Cannot find path to exercises: {:?}", e);
        std::process::exit(2);
    }
    let rootdir = rootdir.unwrap();
    println!("Arguments: {:?}. rootdir: {:?}", args, rootdir);
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
    let mut seen: HashMap<PathBuf, bool> = HashMap::new();
    let mut maybe_root_dir = find_in_ancestors(std::env::current_dir()?, &DAYS[0].dir, uid, &mut seen);
    if maybe_root_dir.is_err() {
        // search again, from program
        maybe_root_dir = find_in_ancestors(PathBuf::from(std::env::args().next().unwrap()).canonicalize()?, &DAYS[0].dir, uid, &mut seen);
    }
    return Ok(maybe_root_dir?);
}

fn find_in_ancestors(startdir: PathBuf, target: &str, uid: u32, seen: &mut HashMap<PathBuf, bool>) -> std::io::Result<PathBuf> {
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
        e.or_insert(true);
    }
    // if we get here, we didn't find it
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Searched all the way to the top, nothing found"))
}
