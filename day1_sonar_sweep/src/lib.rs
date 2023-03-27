use aoc::ExRunner;
use std::io::BufRead;

fn count_increase(acc: (usize, Option<u32>), elem: u32) -> (usize, Option<u32>) {
    if acc.1.is_none() {
        (acc.0, Some(elem))
    } else if acc.1.unwrap() < elem {
        (acc.0 + 1, Some(elem))
    } else {
        (acc.0, Some(elem))
    }
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let in_iter = input.lines()
                    .map(|x| x.unwrap().parse::<u32>().expect("Input should be ints"));
    let part1 = in_iter.fold((0, None), count_increase).0;
    er.part1(part1, Some("number of times depth measurement increases"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"199
200
208
210
200
207
240
269
260
263
".as_bytes()
        )
    }

    #[test]
    fn test_part1() {
        let er = ExRunner::run("day 1 - sonar sweep".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("7".to_string()));
    }
}
