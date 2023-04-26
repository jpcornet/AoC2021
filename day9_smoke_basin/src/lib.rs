use exrunner::ExRunner;
use std::io::BufRead;

fn parse(input: impl BufRead) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.expect("Error reading input").as_bytes().iter().map(|b| *b - ('0' as u8)).collect()).collect()
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let heightmap = parse(input);
    er.parse_done();
    let mut total_risk = 0;
    for y in 0 .. heightmap.len() {
        for x in 0 .. heightmap[y].len() {
            let h = heightmap[y][x];
            if y > 0 && h > heightmap[y-1][x] {
                continue;
            } else if x > 0 && h > heightmap[y][x-1] {
                continue;
            } else if y < heightmap.len() - 1 && h > heightmap[y+1][x] {
                continue;
            } else if x < heightmap[y].len() - 1 && h > heightmap[y][x+1] {
                continue;
            }
            total_risk += h + 1;
        }
    }
    er.part1(total_risk, None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new("2199943210
3987894921
9856789892
8767896789
9899965678
".as_bytes())
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse(BufReader::new("123\n456\n".as_bytes())), vec![vec![1,2,3], vec![4,5,6]]);
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 9 - smoke basin".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("15".to_string()));
    }
}