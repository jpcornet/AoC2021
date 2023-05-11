use exrunner::ExRunner;
use std::{io::BufRead, collections::HashMap};

fn init_braces() -> HashMap<u8, u8> {
    let mut res = HashMap::new();
    res.insert(b'(', b')');
    res.insert(b'[', b']');
    res.insert(b'{', b'}');
    res.insert(b'<', b'>');
    res
}

fn init_scores() -> HashMap<u8, i32> {
    let mut res = HashMap::new();
    res.insert(b')', 3);
    res.insert(b']', 57);
    res.insert(b'}', 1197);
    res.insert(b'>', 25137);
    res
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let braces = init_braces();
    let scores = init_scores();
    let mut total_score = 0;
    for l in input.lines() {
        let nav = l.expect("Error reading input");
        let br = brace_matching(&nav, &braces);
        er.debugln(&format!("Line = {}, matching brace = {:?}", nav, br));
        if let Some(failbr) = br {
            if let Some(sc) = scores.get(&failbr) {
                total_score += *sc;
            } else {
                er.debugln(&format!("Warning: non-matching closing brace {} found", failbr as char));
            }
        }
    }
    er.part1(total_score, None);
}

fn brace_matching(input: &String, braces: &HashMap<u8, u8>) -> Option<u8> {
    let mut stack = Vec::new();
    for c in input.as_bytes() {
        if let Some(brmatch) = braces.get(c) {
            stack.push(*brmatch);
        } else if !stack.is_empty() && *c == stack[stack.len()-1] {
            stack.pop();
        } else {
            return Some(*c);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
".as_bytes())
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run_stderr("day 10 - syntax scoring".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("26397".to_string()));
    }
}
