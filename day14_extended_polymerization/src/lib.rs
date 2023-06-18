use exrunner::ExRunner;
use std::io::BufRead;
use std::collections::HashMap;

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
".as_bytes())
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 14 - extended polymerization".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("1588".to_string()));
    }

}
