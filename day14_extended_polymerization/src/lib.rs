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
        let k = from.trim().to_string();
        assert_eq!(k.len(), 2, "Input pairs should be 2 chars");
        rules.insert(k, to.trim().chars().next().expect("Invalid input, empty insertion"));
    }
    PolyIn { polymers, rules }
}

fn to_mutation(rules: HashMap<String, char>) -> HashMap<String, (String, String)> {
    let mut ret = HashMap::new();
    for (pair, ins) in rules {
        let mut c = pair.chars();
        let newpair1 = format!("{}{ins}", c.next().unwrap());
        let newpair2 = format!("{ins}{}", c.next().unwrap());
        ret.insert(pair, (newpair1, newpair2));
    }
    ret
}

fn count_pairs(p: &str) -> HashMap<String, i64> {
    let mut ret = HashMap::new();
    let mut prev = None;
    for c in p.chars() {
        if let Some(pc) = prev {
            let pair = format!("{pc}{c}");
            ret.entry(pair).and_modify(|x| *x = *x + 1).or_insert(1);
        }
        prev = Some(c);
    }
    ret
}

fn do_polymerize(fcounts: HashMap<String, i64>, polyfreq: &HashMap<String, (String, String)>) -> HashMap<String, i64> {
    let mut nfreq = HashMap::new();
    for (pair, cnt) in fcounts {
        let (npair1, npair2) = polyfreq.get(&pair).expect("Incomplete polymerization rules");
        for np in [npair1, npair2] {
            nfreq.entry(np.to_string()).and_modify(|c| *c = *c + cnt).or_insert(cnt);
        }
    }
    nfreq
}

fn pair_to_polyfreq(pcounts: &HashMap<String, i64>, orig: &str) -> HashMap<char, i64> {
    let mut ret: HashMap<char, i64> = HashMap::new();
    for (pair, cnt) in pcounts {
        let left = pair.chars().next().unwrap();
        ret.entry(left).and_modify(|c| *c += *cnt).or_insert(*cnt);
    }
    // the very last nucleotide in the polymer is not counted this way. Explicitly add that one too
    let last = orig.chars().last().unwrap();
    ret.entry(last).and_modify(|c| *c += 1).or_insert(1);
    ret
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let polyin = parse(input);
    let polyfreq = to_mutation(polyin.rules);
    let mut pfreq = count_pairs(&polyin.polymers);
    er.parse_done();
    for _ in 1..=10 {
        pfreq = do_polymerize(pfreq, &polyfreq);
    }
    let polycount = pair_to_polyfreq(&pfreq, &polyin.polymers);
    er.debugln(&format!("Got: {:?}", polycount));
    let min = polycount.values().min().unwrap();
    let max = polycount.values().max().unwrap();
    er.part1(*max - *min, Some(&format!("Max {}, min {}", *max, *min)));
    for _ in 11..=40 {
        pfreq = do_polymerize(pfreq, &polyfreq);
    }
    let polycount = pair_to_polyfreq(&pfreq, &polyin.polymers);
    er.debugln(&format!("Got: {:?}", polycount));
    let min = polycount.values().min().unwrap();
    let max = polycount.values().max().unwrap();
    er.part2(*max - *min, Some(&format!("Max {}, min {}", *max, *min)));
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
        assert_eq!(er.answ()[1], Some("2188189693529".to_string()));
    }

}
