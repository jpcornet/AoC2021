use exrunner::ExRunner;
use std::io::{BufRead, read_to_string};
use std::collections::HashMap;

fn parse(input: impl BufRead) -> Vec<i32> {
    read_to_string(input).expect("Error reading input").trim()
        .split(',').map(|i| i.parse::<i32>().expect("Input should be numbers")).collect()
}

fn fuel_part2(target: i32, numbers: &Vec<i32>) -> i32 {
    numbers.iter().map(|n| { let d = (target - *n).abs(); d * (d+1) / 2}).sum()
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let mut numbers = parse(input);
    er.parse_done();
    numbers.sort_unstable();
    // take the median == center number. If there is an even number of elements, then any of the middle 2 elements will do
    let aim = numbers[(numbers.len() - 1) / 2];
    er.part1(numbers.iter().map(|n| (aim - *n).abs()).sum::<i32>(), Some(&format!("Aim {aim}, energy level")));
    // start with average, answer is probably around it.
    let avg = (numbers.iter().sum::<i32>() + ((numbers.len() / 2) as i32)) / (numbers.len() as i32);
    // start with a range around average
    let mut start = avg - 1;
    let mut end = avg + 1;
    let mut fuel_at: HashMap<i32, i32> = HashMap::new();
    loop {
        for target in start..=end {
            fuel_at.entry(target).or_insert_with(|| fuel_part2(target, &numbers));
        }
        // determine minimum and position that minimum occurs
        let (min_at, min_fuel) = fuel_at.iter().reduce(|acc, e| if acc.1 < e.1 { acc } else { e } ).unwrap();
        if *min_at == start {
            start -= 2;
        } else if *min_at == end {
            end += 2;
        } else {
            er.part2(*min_fuel, None);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new("16,1,2,0,4,2,7,1,2,14
".as_bytes())
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 7 - The Treachery of Whales".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("37".to_string()));
        assert_eq!(er.answ()[1], Some("168".to_string()));
    }
}
