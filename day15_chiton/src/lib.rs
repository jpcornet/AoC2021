use exrunner::ExRunner;
use std::io::BufRead;

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
".as_bytes())
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 15 - chiton".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("40".to_string()));
    }

}
