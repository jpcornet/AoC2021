use exrunner::ExRunner;
use std::io::BufRead;
use std::collections::HashMap;
use std::str;

fn init_braces() -> HashMap<u8, u8> {
    let mut res = HashMap::new();
    res.insert(b'(', b')');
    res.insert(b'[', b']');
    res.insert(b'{', b'}');
    res.insert(b'<', b'>');
    res
}

fn init_scores1() -> HashMap<u8, i32> {
    let mut res = HashMap::new();
    res.insert(b')', 3);
    res.insert(b']', 57);
    res.insert(b'}', 1197);
    res.insert(b'>', 25137);
    res
}

fn init_scores2() -> HashMap<u8, i32> {
    let mut res = HashMap::new();
    res.insert(b')', 1);
    res.insert(b']', 2);
    res.insert(b'}', 3);
    res.insert(b'>', 4);
    res
}

#[derive(Debug)]
enum BraceParsed {
    Ok,
    FailChar(u8),
    Incomplete(String),
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let braces = init_braces();
    let scores1 = init_scores1();
    let scores2 = init_scores2();
    let mut total_score = 0;
    let mut completion_scores = Vec::new();
    for l in input.lines() {
        let nav = l.expect("Error reading input");
        let br = brace_matching(&nav, &braces);
        er.debugln(&format!("Line = {}, matching brace = {:?}", nav, br));
        match br {
            BraceParsed::Ok => er.debugln(&("Somehow a complete line: ".to_string() + &nav)),
            BraceParsed::FailChar(failbr) => {
                if let Some(sc) = scores1.get(&failbr) {
                    total_score += *sc;
                } else {
                    er.debugln(&format!("Warning: non-matching closing brace {} found", failbr as char));
                }
            },
            BraceParsed::Incomplete(comp) => completion_scores.push(complete_score(&comp, &scores2, er)),
        };
    }
    er.part1(total_score, None);
    completion_scores.sort();
    er.part2(completion_scores[(completion_scores.len()-1)/2], None);
}

fn complete_score(comp: &str, scores: &HashMap<u8, i32>, er: &mut ExRunner) -> i64 {
    let mut score: i64 = 0;
    for c in comp.as_bytes() {
        score *= 5;
        if let Some(sc) = scores.get(c) {
            score += *sc as i64;
        } else {
            er.debugln(&format!("Non-matching completion char: {}", *c as char));
        }
    }
    er.debugln(&format!("Completion score for {comp} = {score}"));
    score
}

fn brace_matching(input: &String, braces: &HashMap<u8, u8>) -> BraceParsed {
    let mut stack = Vec::new();
    for c in input.as_bytes() {
        if let Some(brmatch) = braces.get(c) {
            stack.push(*brmatch);
        } else if !stack.is_empty() && *c == stack[stack.len()-1] {
            stack.pop();
        } else {
            return BraceParsed::FailChar(*c);
        }
    }
    if stack.is_empty() {
        return BraceParsed::Ok
    } else {
        stack.reverse();
        let completion = str::from_utf8(&stack).unwrap().to_string();
        return BraceParsed::Incomplete(completion);
    }
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
        let er = ExRunner::run("day 10 - syntax scoring".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("26397".to_string()));
        assert_eq!(er.answ()[1], Some("288957".to_string()));
    }
}
