use exrunner::ExRunner;
use std::io::BufRead;
use std::collections::HashMap;

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

fn find_7seg_pattern(ssd: &SSDisplay) -> HashMap<u8, u8> {
    // find out which digit is what. First the easy ones...
    let mut seg2num: HashMap<u8, u8> = HashMap::new();
    let mut opt_seg_cf = None;
    let mut opt_seg_bd = None;
    for pat in ssd.patterns {
        match pat.count_ones() {
            2 => { seg2num.insert(pat, 1); opt_seg_cf = Some(pat); },
            3 => { seg2num.insert(pat, 7); },
            4 => { seg2num.insert(pat, 4); opt_seg_bd = Some(pat); },
            7 => { seg2num.insert(pat, 8); },
            _ => (),
        };
    }
    assert!(opt_seg_cf.is_some());
    assert!(opt_seg_bd.is_some());
    let seg_cf = opt_seg_cf.unwrap();
    // opt_seg_bd really contains bdcf, so remote cf.
    let seg_bd = opt_seg_bd.unwrap() & !seg_cf;
    // scan the 6-segment patterns. If one of cf is missing, it's a 6 and the missing one is c.
    // if one of bd is missing, it's a 0, and b is the one that's present in the pattern.
    // the remaining 6-lit pattern is 9 and the missing one is e.
    let mut opt_seg_b = None;
    let mut opt_seg_c = None;
    let mut opt_seg_e = None;
    let mut opt_seg_f = None;
    // just for reference (but not needed here):
    // seg_a is simply pattern for 7 minus seg_cf
    // seg_d is the one that's missing in 0 or seg_bd minus seg_b
    // seg_g is the one that is left.
    for pat in ssd.patterns.iter().filter(|p| p.count_ones() == 6) {
        if *pat & seg_cf != seg_cf {
            seg2num.insert(*pat, 6);
            opt_seg_c = Some(seg_cf & !*pat);
            opt_seg_f = Some(seg_cf & *pat);
        } else if *pat & seg_bd != seg_bd {
            seg2num.insert(*pat, 0);
            opt_seg_b = Some(seg_bd & *pat);
        } else {
            seg2num.insert(*pat, 9);
            opt_seg_e = Some(0x7f & !*pat);
        }
    }
    assert!(opt_seg_b.is_some());
    assert!(opt_seg_c.is_some());
    assert!(opt_seg_e.is_some());
    assert!(opt_seg_f.is_some());
    // now look at the 5-segment patterns. If bf is missing, it's a 2,
    // if be is missing, it's a 3, if ce is missing it's a 5.
    for pat in ssd.patterns.iter().filter(|p| p.count_ones() == 5) {
        if *pat & (opt_seg_b.unwrap() | opt_seg_f.unwrap()) == 0 {
            seg2num.insert(*pat, 2);
        } else if *pat & (opt_seg_b.unwrap() | opt_seg_e.unwrap()) == 0 {
            seg2num.insert(*pat, 3);
        } else if *pat & (opt_seg_c.unwrap() | opt_seg_e.unwrap()) == 0 {
            seg2num.insert(*pat, 5);
        } else {
            panic!("Invalid 7-segment patterns, unexpected 5-seg pattern");
        }
    }
    // we should have all 10 numbers now.
    assert_eq!(seg2num.len(), 10);
    seg2num
}

fn decode_digits(ssd: &SSDisplay) -> u32 {
    let decode = find_7seg_pattern(ssd);
    let mut answ: u32 = 0;
    for dpat in ssd.digits {
        answ = answ * 10 + *decode.get(&dpat).expect("Unknown pattern in digits") as u32;
    }
    answ
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let inlist = parse(input);
    er.parse_done();
    // do part1, count number of digits that are 1, 4, 7, or 8 (2, 4, 3, 7 segmenets lit)
    let answ1 = inlist.iter().map(|ssd| ssd.digits.iter()).flatten()
        .filter(|d| match d.count_ones() { 2..=4 => true, 7 => true, _ => false})
        .count();
    er.part1(answ1, None);
    let answ2: u32 = inlist.iter().map(|ssd| decode_digits(ssd)).sum();
    er.part2(answ2, None);
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

    fn simple_test_input() -> BufReader<&'static [u8]> {
        BufReader::new("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".as_bytes())
    }

    #[test]
    fn test_find_7seg() {
        let testin = parse(simple_test_input());
        let s7 = find_7seg_pattern(&testin[0]);
        assert_eq!(s7.get(&0x3), Some(&1)); // ab
        assert_eq!(s7.get(&0x3F), Some(&9)); // cefabd
        assert_eq!(s7.get(&0x3E), Some(&5)); // cdfbe
        assert_eq!(s7.get(&0x2F), Some(&3)); // fbcad
    }

    #[test]
    fn test_decode_single() {
        let testin = parse(simple_test_input());
        assert_eq!(decode_digits(&testin[0]), 5353);
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
        assert_eq!(er.answ()[1], Some("61229".to_string()));
    }
}
