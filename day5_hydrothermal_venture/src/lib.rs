use exrunner::ExRunner;
use std::{io, io::BufRead, collections::{HashMap, hash_map::Entry}};

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
".as_bytes()
        )
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 5 - hydrothermal venture".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("5".to_string()));
    }

}
