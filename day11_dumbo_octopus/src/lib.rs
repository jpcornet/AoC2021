use exrunner::ExRunner;
use std::io::BufRead;

#[derive(Debug)]
struct Octogy {
    level: u8,
    flashed: bool,
}

fn parse(input: impl BufRead) -> Vec<Vec<Octogy>> {
    input.lines().map(|l| l.unwrap().as_bytes().iter().map(|c| Octogy { level: *c - b'0', flashed: false }).collect()).collect()
}

fn do_one_step(octopii: &mut Vec<Vec<Octogy>>) -> i32 {
    let mut process_flash: Vec<(usize, usize)> = Vec::new();
    for y in 0 .. octopii.len() {
        for x in 0 .. octopii[y].len() {
            octopii[y][x].level += 1;
            if octopii[y][x].level > 9 {
                octopii[y][x] = Octogy { level: 0, flashed: true };
                process_flash.push((x, y));
            } else {
                octopii[y][x].flashed = false;
            }
        }
    }
    let mut flashes = 0;
    while !process_flash.is_empty() {
        flashes += 1;
        let (x, y) = process_flash.pop().unwrap();
        for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
            if (y as i32) + dy < 0 || (y as i32) + dy >= octopii.len() as i32 || (x as i32) + dx < 0 || (x as i32) + dx >= octopii[y].len() as i32 {
                continue;
            }
            let nx = ((x as i32) + dx) as usize;
            let ny = ((y as i32) + dy) as usize;
            if !octopii[ny][nx].flashed {
                octopii[ny][nx].level += 1;
                if octopii[ny][nx].level > 9 {
                    octopii[ny][nx] = Octogy { level: 0, flashed: true };
                    process_flash.push((nx, ny));
                }
            }
        }
    }
    flashes
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let mut octopii = parse(input);
    er.parse_done();
    let mut flashes: i32 = 0;
    for _ in 1..=100 {
        flashes += do_one_step(&mut octopii);
    }
    er.debugln(&format!("Got: {:?}", octopii));
    er.part1(flashes, Some("Number of flashes"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
".as_bytes())
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 11 - dumbo octopus".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("1656".to_string()));
    }
}
