use exrunner::ExRunner;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
struct PolyIn {
    polymers: String,
    rules: HashMap<String, char>,
}

fn parse(input: impl BufRead) -> PolyIn {
    let mut lines = input.lines();
    let polymers = lines.next().expect("Input cannot be empty").expect("Error reading input");
    let mut rules = HashMap::new();
    for l in lines {
        let line = l.expect("Error reading input");
        if line.len() == 0 {
            continue;
        }
        let (from, to) = line.split_once("->").expect("Invalid input");
        rules.insert(from.trim().to_string(), to.trim().chars().next().expect("Invalid input, empty insertion"));
    }
    PolyIn { polymers, rules }
}

fn do_polymerize(p: &mut PolyIn) {
    let mut prev: Option<char> = None;
    p.polymers = p.polymers.chars().map(|c| {
        let ret;
        if let Some(pc) = prev {
            let from = format!("{pc}{c}");
                let insert = p.rules.get(&from).expect("Incomplete polymerization rules");
            ret = format!("{insert}{c}");
        } else {
            ret = c.to_string();
        }
        prev = Some(c);
        ret
    }).collect();
}

fn count_poly(p: &str) -> HashMap<char, usize> {
    let mut cnt = HashMap::new();
    for c in p.chars() {
        cnt.entry(c).and_modify(|x| *x = *x + 1).or_insert(1);
    }
    cnt
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let mut polyin = parse(input);
    er.parse_done();
    for _ in 1..=10 {
        do_polymerize(&mut polyin);
    }
    let counts = count_poly(&polyin.polymers);
    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();
    // just debugging
    let minc = counts.keys().filter(|c| counts.get(c).unwrap() == min).next().unwrap();
    let maxc = counts.keys().filter(|c| counts.get(c).unwrap() == max).next().unwrap();
    er.part1(max-min, Some(&format!("Max {maxc} occurs {max}, Min {minc} occurs {min}")));
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
