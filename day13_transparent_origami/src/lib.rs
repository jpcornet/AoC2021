use exrunner::ExRunner;
use std::io::BufRead;
use std::collections::HashMap;
use std::str;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

#[derive(Debug, Default)]
struct OrigamiInstructions {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

fn parse(input: impl BufRead) -> OrigamiInstructions {
    let mut ret = OrigamiInstructions::default();
    let origami_re = Regex::new(r"(?x)
        ^(
            (?<X>\d+),(?<Y>\d+)
            |
            fold\salong\s(?<fold>[xy])=(?<at>\d+)
        )$
        ").unwrap();
    for l in input.lines() {
        let line = l.expect("Error reading input");
        if line.len() == 0 {
            // skip empty lines
            continue;
        }
        let cap = origami_re.captures(&line).expect("Cannot parse input");
        if let Some(xm) = cap.name("X") {
            let x: i32 = xm.as_str().parse().unwrap();
            let y: i32 = cap["Y"].parse().unwrap();
            ret.points.push(Point{x, y});
        } else {
            let at: i32 = cap["at"].parse().unwrap();
            ret.folds.push(
                match &cap["fold"] {
                    "x" => Fold::X(at),
                    "y" => Fold::Y(at),
                    _ => panic!("Logic error in regex match"),
                }
            );
        }
    }
    ret
}

fn do_fold(points: &mut HashMap<Point, ()>, f: &Fold) {
    let mut replaces = Vec::new();
    for p in points.keys() {
        let opt_newp = match f {
            Fold::X(limit) => {
                if p.x > *limit {
                    Some(Point{ x: 2 * *limit - p.x, y: p.y })
                } else {
                    None
                }
            },
            Fold::Y(limit) => {
                if p.y > *limit {
                    Some(Point{ x: p.x, y: 2 * *limit - p.y })
                } else {
                    None
                }
            }
        };
        if let Some(newp) = opt_newp {
            replaces.push(((*p).clone(), newp));
        }
    }

    for (p, newp) in replaces {
        points.remove(&p);
        points.insert(newp, ());
    }
}

fn draw_points(points: Vec<Point>) -> String {
    let (minx, maxx, miny, maxy) = points.iter().fold((i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(mut minx, mut maxx, mut miny, mut maxy), p| {
            if p.x < minx {
                minx = p.x;
            }
            if p.x > maxx {
                maxx = p.x;
            }
            if p.y < miny {
                miny = p.y;
            }
            if p.y > maxy {
                maxy = p.y;
            }
            (minx, maxx, miny, maxy)
        });
    let mut canvas: Vec<Vec<u8>> = Vec::with_capacity((maxy-miny+1) as usize);
    let mut xvec = Vec::with_capacity((maxx-minx+1) as usize);
    for _x in minx..=maxx {
        xvec.push(b'.');
    }
    for _y in miny..=maxy {
        canvas.push(xvec.clone());
    }
    for p in points {
        canvas[(p.y - miny) as usize][(p.x - minx) as usize] = b'#';
    }
    canvas.into_iter().map(|xv| str::from_utf8(&xv).unwrap().to_string()).collect::<Vec<String>>().join("\n")
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let instr = parse(input);
    er.parse_done();
    // convert points to hashmap
    let mut field: HashMap<Point, ()> = HashMap::new();
    for p in instr.points {
        field.insert(p, ());
    }
    let mut folds = instr.folds.iter();
    do_fold(&mut field, &folds.next().unwrap());
    er.part1(field.len(), None);
    for f in folds {
        do_fold(&mut field, f);
    }
    let disp = draw_points(field.into_keys().collect());
    er.part2(disp, Some("folded orgami output"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
".as_bytes()
        )
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 13 - transparent origami".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("17".to_string()));
    }
}
