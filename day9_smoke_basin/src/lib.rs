use exrunner::ExRunner;
use std::{io::BufRead, collections::HashMap};

fn parse(input: impl BufRead) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.expect("Error reading input").as_bytes().iter().map(|b| *b - ('0' as u8)).collect()).collect()
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let heightmap = parse(input);
    er.parse_done();
    let mut total_risk: u32 = 0;
    let mut basins: Vec<usize> = Vec::new();
    for y in 0 .. heightmap.len() {
        for x in 0 .. heightmap[y].len() {
            let h = heightmap[y][x];
            if y > 0 && h >= heightmap[y-1][x] {
                continue;
            } else if x > 0 && h >= heightmap[y][x-1] {
                continue;
            } else if y < heightmap.len() - 1 && h >= heightmap[y+1][x] {
                continue;
            } else if x < heightmap[y].len() - 1 && h >= heightmap[y][x+1] {
                continue;
            }
            total_risk += (h + 1) as u32;
            let bsize = get_basin_size(&heightmap, x, y);
            er.debugln(&format!("Low point at {x},{y} = {h}, basin size={bsize}"));
            basins.push(bsize);
        }
    }
    er.part1(total_risk, None);
    basins.sort();
    er.part2(basins.into_iter().rev().take(3).fold(1, |acc, e| acc * e), None);
}

fn get_basin_size(hmap: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut basinpoints: HashMap<(i32, i32), ()> = HashMap::new();
    let mut consider: Vec<(i32, i32)> = Vec::new();
    consider.push((x as i32, y as i32));
    basinpoints.insert((x as i32, y as i32), ());
    while !consider.is_empty() {
        let (xc, yc) = consider.pop().unwrap();
        let refh = hmap[yc as usize][xc as usize];
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if yc + dy < 0 || yc + dy >= hmap.len() as i32 || xc + dx < 0 || xc + dx >= hmap[(yc+dy) as usize].len() as i32 {
                // out of field
                continue;
            }
            let (xn, yn) = ((xc+dx) as usize, (yc+dy) as usize);
            if hmap[yn][xn] == 9 || hmap[yn][xn] < refh {
                // 9 isn't part of basin, and basin cannot go down
                continue;
            }
            let newp = (xn as i32, yn as i32);
            basinpoints.entry(newp).or_insert_with(|| {consider.push(newp); ()} );
        }
    }
    basinpoints.len()
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
        assert_eq!(er.answ()[1], Some("1134".to_string()));
    }
}
