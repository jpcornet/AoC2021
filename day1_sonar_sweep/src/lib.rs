use exrunner::ExRunner;
use std::io::BufRead;

fn count_increase(acc: (usize, Option<u32>), elem: &u32) -> (usize, Option<u32>) {
    let newcount = if acc.1.is_none() || *elem <= acc.1.unwrap() {
        acc.0
    } else {
        acc.0 + 1
    };
    (newcount, Some(*elem))
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let in_vec: Vec<u32> = input.lines()
                    .map(|x| x.unwrap().parse::<u32>().expect("Input should be ints")).collect();
    er.parse_done();
    let part1 = in_vec.iter().fold((0, None), count_increase).0;
    er.part1(part1, Some("number of times depth measurement increases"));
    let part2 = (3..in_vec.len()).filter(|i| in_vec[*i-3] < in_vec[*i]).count();
    er.part2(part2, None);
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
    fn test_part12() {
        let er = ExRunner::run("day 1 - sonar sweep".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("7".to_string()));
        assert_eq!(er.answ()[1], Some("5".to_string()));
    }
}
