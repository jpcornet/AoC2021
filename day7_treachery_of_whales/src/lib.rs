use exrunner::ExRunner;
use std::io::{BufRead, read_to_string};

fn parse(input: impl BufRead) -> Vec<i32> {
    read_to_string(input).expect("Error reading input").trim()
        .split(',').map(|i| i.parse::<i32>().expect("Input should be numbers")).collect()
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let mut numbers = parse(input);
    er.parse_done();
    numbers.sort_unstable();
    // take the medium == center number. If there is an even number of elements, then any of the middle 2 elements will do
    let aim = numbers[(numbers.len() - 1) / 2];
    er.part1(numbers.iter().map(|n| (aim - *n).abs()).sum::<i32>(), Some(&format!("Aim {aim}, energy level")));
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
        //assert_eq!(er.answ()[1], Some("".to_string()));
    }
}
