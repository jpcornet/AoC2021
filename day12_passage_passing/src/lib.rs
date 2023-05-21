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
        assert_ne!(a, b, "Caves cannot connect to themselves");
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
    // routes will contain the list of possible routes, starts out with just "start".
    // routes that do not work out are replace by None
    // if we visited a small room twice, the string starts with "!"
    let mut routes = vec![Some("start".to_string())];
    let mut path1: usize = 0;
    let mut path2: usize = 0;
    while !routes.is_empty() {
        let mut addroutes = Vec::new();
        let mut somes: usize = 0;
        let mut nones: usize = 0;
        for maybe_r in &mut routes {
            if maybe_r.is_none() {
                continue;
            }
            let r = maybe_r.as_ref().unwrap();
            let node = r.rsplit(',').next().unwrap();
            if node == "end" {
                panic!("Logic error, routes at end should not be on routes")
            }
            let cave = caves.get(node).expect(&format!("Error, bad cave {node}"));
            for nxt in &cave.to {
                // we cannot go back to start
                if nxt == "start" {
                    continue;
                } else if nxt == "end" {
                    //er.debugln(&format!("Route: {},{}", r, nxt));
                    if r.starts_with("!") {
                        path2 += 1;
                    } else {
                        path1 += 1;
                    }
                    // remove this path from array now, so just continue
                    continue;
                }
                // if it's a lowercase cave, make sure we haven't visited it already
                let sml_twice = nxt.chars().all(|c| c.is_ascii_lowercase()) && r.contains(&(",".to_string() + nxt + ","));
                // we can only visit one small room twice, so do not add this one if this is the second small room that we visit twice
                if sml_twice && r.starts_with("!") {
                    continue;
                }
                let new_route = format!("{}{},{}", if sml_twice { "!" } else { "" }, r, nxt);
                // addroutes contains Option<String> just to make it compatible with routes. It never contains None.
                addroutes.push(Some(new_route));
            }
            // replace this route with the first route found. Or any route. Or with "None" if there are no routes.
            *maybe_r = addroutes.pop().unwrap_or(None);
            if maybe_r.is_some() {
                somes += 1;
            } else {
                nones += 1;
            }
        }
        // only prune if we inserted enough nones
        if nones * 10 > somes {
            er.debugln(&format!("Routes before prune: {}. Nones={}, Somes={}", routes.len(), nones, somes));
            routes = routes.into_iter().filter(|x| x.is_some()).collect();
            er.debugln(&format!("Routes after prune: {}", routes.len()));
        }
        routes.append(&mut addroutes);
    }
    er.part1(path1, Some("all routes with small caves once"));
    er.part2(path1 + path2, Some("all possible routes"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use exrunner::ExCtx;
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
        assert_eq!(er.answ()[1], Some("36".to_string()));
    }

    #[test]
    fn test_solve2() {
        let er = ExRunner::run("day 12 - passage pathing".to_string(), solve, test_input2());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("19".to_string()));
        assert_eq!(er.answ()[1], Some("103".to_string()));
    }

    #[test]
    fn test_solve3() {
        let ctx = ExCtx::new(solve, test_input3());
        let er = ctx.do_run("day 12 - passage pathing".to_string());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("226".to_string()));
        assert_eq!(er.answ()[1], Some("3509".to_string()));
    }

}
