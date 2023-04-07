use exrunner::ExRunner;
use std::{io::BufRead, str::FromStr, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (xstr, ystr) = input.split_once(',').ok_or(ParsePointError)?;
        let x: i32 = xstr.trim().parse().map_err(|_| ParsePointError)?;
        let y: i32 = ystr.trim().parse().map_err(|_| ParsePointError)?;
        Ok(Point {x, y})
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseLineError;

impl FromStr for  Line {
    type Err = ParseLineError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (startstr, endstr) = input.split_once("->").ok_or(ParseLineError)?;
        let start: Point = startstr.trim().parse().map_err(|_| ParseLineError)?;
        let end: Point = endstr.trim().parse().map_err(|_| ParseLineError)?;
        Ok(Line { start, end })
    }
}

fn parse_input(input: impl BufRead) -> Vec<Line> {
    input.lines().map(|l| l.expect("Error reading input").parse().expect("Error parsing input")).collect()
}

fn doublepoints(lines: &Vec<Line>, do_diagonal: bool) -> i32 {
    let mut online: HashMap<Point, bool> = HashMap::new();
    let mut double_points = 0;
    for l in lines {
        let mut x = l.start.x;
        let mut y = l.start.y;
        let x2 = l.end.x;
        let y2 = l.end.y;
        let dx; let dy;
        if x == x2 {
            dx = 0;
            dy = if y > y2 { -1 } else { 1 };
        } else if y == y2 {
            dx = if x > x2 { -1 } else { 1 };
            dy = 0;
        } else {
            if !do_diagonal {
                continue;
            }
            dx = if x > x2 { -1 } else { 1 };
            dy = if y > y2 { -1 } else { 1 };
            // make sure lines are exactly 45 degrees
            assert_eq!((x-x2)*dx, (y-y2)*dy);
        }
        loop {
            online.entry(Point{ x, y})
                .and_modify(|taken| if *taken { double_points += 1; *taken = false; })
                .or_insert(true);
            if x == x2 && y == y2 {
                break;
            }
            x += dx;
            y += dy;
        }
    }
    double_points
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let lines = parse_input(input);
    er.parse_done();
    er.part1(doublepoints(&lines, false), None);
    er.part2(doublepoints(&lines, true), None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
".as_bytes()
        )
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 5 - hydrothermal venture".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("5".to_string()));
        assert_eq!(er.answ()[1], Some("12".to_string()));
    }

}
