use exrunner::ExRunner;
use std::{io::BufRead, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Hash)]
struct BinDigit {
    weight: u32,
    digit: bool,
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let lines: Vec<String> = input.lines().map(|x| x.unwrap()).collect();
    er.parse_done();
    let mut appear: HashMap<BinDigit, usize> = HashMap::new();
    for l in lines {
        for (p, b) in l.chars().enumerate() {
            let weight: u32 = 1 << (l.len()-1-p);
            let digit = match b {
                '0' => false,
                '1' => true,
                _ => panic!("Invalid digit in input"),
            };
            let cnt = appear.entry(BinDigit{ weight, digit }).or_insert(0);
            *cnt += 1;
        }
    }
    let mut most_common = 0;
    let mut least_common = 0;
    let mut weight = 1;
    loop {
        let ones = appear.get(&BinDigit{ weight, digit: true }).unwrap_or(&0);
        let zeros = appear.get(&BinDigit{ weight, digit: false}).unwrap_or(&0);
        if *ones + *zeros == 0 {
            break;
        }
        if *ones > *zeros {
            most_common += weight;
        } else if *ones < *zeros {
            least_common += weight;
        } else {
            panic!("As many ones as zeros for {weight}, stop");
        }
        weight <<= 1;
    }
    er.part1(most_common as u64 * least_common as u64, Some(&format!("gamma rate={most_common}, epsilon rate={least_common}")));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
".as_bytes()
        )
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 3 - binary diagnostic".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("198".to_string()));
    }
}
