use exrunner::ExRunner;
use std::io::BufRead;
use std::collections::HashMap;

fn parse(input: impl BufRead) -> Vec<Vec<u8>> {
    input.lines().map(|l| {
        l.expect("Error reading input").as_bytes().into_iter().map(|c| *c - b'0').collect() }
    ).collect()
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let field = parse(input);
    assert!(field.len() > 0, "Input should be non-empty");
    assert!(field[0].len() > 0, "Input lines should be non-empty");
    er.parse_done();
    // initialize a known risk factor array
    let mut risk: Vec<Vec<Option<i32>>> = Vec::new();
    let xsize = field[0].len();
    let ysize = field.len();
    for y in 0 .. ysize {
        assert_eq!(field[y].len(), xsize, "Input not square");
        let mut riskrow: Vec<Option<i32>> = Vec::new();
        riskrow.resize(xsize, None);
        risk.push(riskrow);
    }
    // the starting position, upper left, is the beginning
    risk[0][0] = Some(0);
    let mut walkers: Vec<(usize, usize)> = vec![(0, 0)];
    loop {
        // new walkers in a hashmap, as one single point could be inserted twice in this loop,
        // make sure it only appears once in the next run.
        let mut new_walkers: HashMap<(usize, usize), ()> = HashMap::new();
        for (x, y) in walkers {
            let cur_risk = risk[y][x].unwrap();
            // look around in all directions
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if (x as i32) + dx < 0 || (y as i32) + dy < 0 {
                    continue;
                }
                let tx = ((x as i32) + dx) as usize;
                let ty = ((y as i32) + dy) as usize;
                if tx >= xsize || ty >= ysize {
                    continue;
                }
                let new_risk = cur_risk + (field[ty][tx] as i32);
                if let Some(old_risk) = risk[ty][tx] {
                    if old_risk <= new_risk {
                        // this path is not better than what we had, so give up
                        continue;
                    }
                }
                risk[ty][tx] = Some(new_risk);
                new_walkers.insert((tx, ty), ());
            }
        }
        walkers = new_walkers.into_keys().collect();
        // keep walking until all walkers are done
        if walkers.is_empty() {
            break;
        }
    }
    er.part1(risk[ysize-1][xsize-1].unwrap(), None);
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
