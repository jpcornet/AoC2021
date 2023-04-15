use exrunner::ExRunner;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
struct SSDisplay {
    patterns: [u8; 10],
    digits: [u8; 4],
}

fn segments_to_bitmap(segments: &str) -> u8 {
    segments.chars().fold(0, |acc, c| acc | (1 << match c {
        'a' ..= 'g' => (c  as u32) - ('a' as u32),
        _ => panic!("Segments should only use a..g"),
    }))
}

fn parse(input: impl BufRead) -> Vec<SSDisplay> {
    // make sure the input contains of single lines containing both the patterns and the readout
    let mut prevline: Option<String> = None;
    input.lines().filter_map(|l| {
        let line = l.expect("Error reading input");
        if prevline.is_some() {
            Some(prevline.take().unwrap() + &line)
        } else if line.trim_end().ends_with('|') {
            prevline = Some(line);
            None
        } else {
            Some(line)
        }
    })
    // now map each line to a SSDiplay
    .map(|l: String| {
        let (strpatterns, strdigits) = l.split_once('|').expect("Input lines should contain | separator");
        let mut disp = SSDisplay{ patterns: [0; 10], digits: [0; 4] };
        let mut enough = false;
        for (i, bmap) in strpatterns.split_whitespace().map(|seg| segments_to_bitmap(seg)).enumerate() {
            if i == disp.patterns.len() - 1 {
                enough = true;
            } else if i >= disp.patterns.len() {
                panic!("Too many patterns");
            }
            disp.patterns[i] = bmap;
        }
        assert!(enough, "Not enough patterns");
        enough = false;
        for (i, bmap) in strdigits.split_whitespace().map(|seg| segments_to_bitmap(seg)).enumerate() {
            if i == disp.digits.len() - 1 {
                enough = true;
            } else if i >= disp.digits.len() {
                panic!("Too many digits");
            }
            disp.digits[i] = bmap;
        }
        assert!(enough, "Not enough digits");
        disp
    }).collect()
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let inlist = parse(input);
    er.parse_done();
    // do part1, count number of digits that are 1, 4, 7, or 8 (2, 4, 3, 7 segmenets lit)
    let answ1 = inlist.iter().map(|ssd| ssd.digits.iter()).flatten()
        .filter(|d| match d.count_ones() { 2..=4 => true, 7 => true, _ => false})
        .count();
    er.part1(answ1, None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
fgae cfgab fg bagce
".as_bytes())
    }

    #[test]
    fn test_segments_to_bitmap() {
        assert_eq!(segments_to_bitmap("ab"), 0x3);
        assert_eq!(segments_to_bitmap("g"), 0x40);
    }

    #[test]
    fn test_parse() {
        let answer = vec![SSDisplay{ patterns: [ 0x12, 0x7F, 0x7E, 0x7D, 0x56, 0x7C, 0x7B, 0x3E, 0x2F, 0x1A ], digits: [0x7F, 0x3E, 0x7E, 0x56]}];
        assert_eq!(parse(BufReader::new("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".as_bytes())), answer);
        assert_eq!(parse(BufReader::new("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
    fdgacbe cefdb cefbgd gcbe".as_bytes())), answer);
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 8 - Seven Segment Search".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("26".to_string()));
    }
}
