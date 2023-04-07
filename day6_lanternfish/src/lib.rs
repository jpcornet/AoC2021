use exrunner::ExRunner;
use std::io::{BufRead, read_to_string};
use std::collections::HashMap;

fn parse(input: impl BufRead) -> HashMap<u8, usize> {
    let fishes: Vec<u8> = read_to_string(input).expect("Error reading input").trim()
        .split(',').map(|i| i.parse::<u8>().expect("Input should be numbers")).collect();
    let mut population: HashMap<u8, usize> = HashMap::new();
    for f in fishes {
        population.entry(f)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    population
}

fn process_one_day(fishpop: &mut HashMap<u8, usize>) {
    // get 0-count population. Dereference to prevent a borrow of the hashmap
    let popzero = *fishpop.get(&0).unwrap_or(&0);
    for age in 1..=8 {
        // every fish with energy "age" becomes "age - 1"
        if let Some(popsize) = fishpop.get(&age) {
            fishpop.insert(age - 1, *popsize);
        } else {
            fishpop.remove(&(age - 1));
        }
    }
    // all population 0 fishes spanwed popuplation 8 fishes, who should now be gone.
    fishpop.insert(8, popzero);
    // additionally, the original population 0 fishes get added to the popuplation 6 fishes
    fishpop.entry(6)
        .and_modify(|pop6| *pop6 += popzero)
        .or_insert(popzero);
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let mut fishpop = parse(input);
    er.parse_done();
    for _ in 0..80 {
        process_one_day(&mut fishpop);
    }
    er.part1(fishpop.values().map(|v| *v as u64).sum::<u64>(), None);
    for _ in 80..256 {
        process_one_day(&mut fishpop);
    }
    er.part2(fishpop.values().map(|v| *v as u64).sum::<u64>(), None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new("3,4,3,1,2
".as_bytes())
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 6 - lanternfish".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("5934".to_string()));
        assert_eq!(er.answ()[1], Some("26984457539".to_string()));
    }
}
