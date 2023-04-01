use exrunner::ExRunner;
use std::io::BufRead;

enum Movement {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn parse(input: impl BufRead) -> Vec<Movement> {
    input.lines().map(|l| {
        let line = l.expect("Error reading input");
        let sp = line.find(char::is_whitespace).expect("Input line should contain space");
        let amount: i64 = line[sp..].trim().parse().expect("Input should contain numbers");
        match &line[..sp] {
            "forward" => Movement::Forward(amount),
            "down" => Movement::Down(amount),
            "up" => Movement::Up(amount),
            _ => panic!("Unknown movement type"),
        }
    }).collect()
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let ivec = parse(input);
    er.parse_done();
    let mut h: i64 = 0;
    let mut d: i64 = 0;
    let mut aim: i64 = 0;
    let mut d2: i64 = 0;
    for m in ivec {
        match m {
            Movement::Forward(a) => { h += a; d2 += aim * a },
            Movement::Down(a) => { d += a; aim += a },
            Movement::Up(a) => { d -= a; aim -= a },
        };
    }
    er.part1(h*d, Some(&format!("Position horizonatl={h}, depth={d}. Multiplied:")));
    er.part2(h*d2, Some(&format!("part2 horizontal={h}, depth={d2}. Multiplied:")));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"forward 5
down 5
forward 8
up 3
down 8
forward 2
".as_bytes()
        )
    }

    #[test]
    fn test() {
        let er = ExRunner::run("day 2 - dive".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("150".to_string()));
        assert_eq!(er.answ()[1], Some("900".to_string()));
    }
}
