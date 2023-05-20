use exrunner::ExRunner;
use std::{io::BufRead, collections::HashMap};

#[derive(Debug)]
struct Cave {
    to: Vec<String>,
}

fn parse(input: impl BufRead) -> HashMap<String, Cave> {
    let mut ret = HashMap::new();
    for l in input.lines() {
        let lstr = l.expect("Error reading input");
        let (a, b) = lstr.split_once('-').expect("Input lines should contain - char");
        for (one, two) in [ (a, b), (b, a) ] {
            ret.entry(one.to_string())
                .and_modify(|cave: &mut Cave| cave.to.push(two.to_string()))
                .or_insert(Cave{ to: vec![two.to_string()]});
        }
    }
    ret
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let caves = parse(input);
    er.parse_done();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input1() -> BufReader<&'static [u8]> {
        BufReader::new(
"start-A
start-b
A-c
A-b
b-d
A-end
b-end
".as_bytes()
        )
    }

    fn test_input2() -> BufReader<&'static [u8]> {
        BufReader::new(
"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
".as_bytes()
        )
    }

    fn test_input3() -> BufReader<&'static [u8]> {
        BufReader::new(
"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
".as_bytes()
        )
    }

    #[test]
    fn test_solve1() {
        let er = ExRunner::run("day 12 - passage pathing".to_string(), solve, test_input1());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("10".to_string()));
    }

    #[test]
    fn test_solve2() {
        let er = ExRunner::run("day 12 - passage pathing".to_string(), solve, test_input2());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("19".to_string()));
    }

    #[test]
    fn test_solve3() {
        let er = ExRunner::run("day 12 - passage pathing".to_string(), solve, test_input3());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("226".to_string()));
    }

}
