use exrunner::ExRunner;
use std::{io::BufRead, collections::HashMap};
use std::str;

#[derive(Debug, PartialEq, Eq, Hash)]
struct BinDigit {
    weight: u32,
    digit: bool,
}

fn reduce_list_bitcrit<'a>(lines: &Vec<&'a [u8]>, bitpos: u32, most: bool) -> &'a [u8] {
    let mut occur: HashMap<u8, usize> = HashMap::new();
    for l in lines {
        let pos = (l.len() as i32) - 1 - (bitpos as i32);
        if pos >= 0 && pos < l.len() as i32 {
            occur.entry(l[pos as usize]).and_modify(|x| *x += 1).or_insert(1);
        }
    }
    if occur.is_empty() {
        panic!("Bit position {bitpos} is never used");
    }
    let ones = occur.get(&b'1').unwrap_or(&0);
    let zeros = occur.get(&b'0').unwrap_or(&0);
    // assuming most common, which value to keep.
    let mut keep = if ones >= zeros { b'1' } else { b'0' };
    // if not the most, we keep the other value
    if !most {
        keep = keep ^ 1u8;
    }
    let mut reduced: Vec<&[u8]> = Vec::new();
    for l in lines {
        let pos = (l.len() as i32) - 1 - (bitpos as i32);
        if pos >= 0 && pos < l.len() as i32 && l[pos as usize] == keep {
            reduced.push(*l);
        }
    }
    match reduced.len() {
        0 => panic!("Filtering on position {bitpos} resulted in zero occurrances."),
        1 => reduced[0],
        _ => reduce_list_bitcrit(&reduced, bitpos - 1, most),
    }
}

pub fn solve(mut input: impl BufRead, er: &mut ExRunner) {
    let mut inbuf: Vec<u8> = Vec::new();
    input.read_to_end(&mut inbuf).expect("Error reading input file");
    let lines: Vec<&[u8]> = inbuf.split(|f| *f == b'\n').collect();
    er.parse_done();
    let mut appear: HashMap<BinDigit, usize> = HashMap::new();
    for l in lines.iter() {
        for (p, b) in l.iter().enumerate() {
            let weight: u32 = 1 << (l.len()-1-p);
            let digit = match b {
                b'0' => false,
                b'1' => true,
                _ => panic!("Invalid digit in input"),
            };
            appear.entry(BinDigit{ weight, digit }).and_modify(|x| *x += 1).or_insert(1);
        }
    }
    let mut most_common = 0;
    let mut least_common = 0;
    let mut weight = 1;
    loop {
        let ones = appear.get(&BinDigit{ weight, digit: true }).unwrap_or(&0);
        let zeros = appear.get(&BinDigit{ weight, digit: false}).unwrap_or(&0);
        if *ones + *zeros == 0 {
            break;
        }
        if *ones > *zeros {
            most_common += weight;
        } else if *ones < *zeros {
            least_common += weight;
        } else {
            panic!("As many ones as zeros for {weight}, stop");
        }
        weight <<= 1;
    }
    er.part1(most_common as u64 * least_common as u64, Some(&format!("gamma rate={most_common}, epsilon rate={least_common}")));
    let startweight = weight.trailing_zeros() - 1;
    let oxygen = reduce_list_bitcrit(&lines, startweight, true);
    let co2 = reduce_list_bitcrit(&lines, startweight, false);
    let oxygen_i = i64::from_str_radix(str::from_utf8(oxygen).unwrap(), 2).unwrap();
    let co2_i = i64::from_str_radix(str::from_utf8(co2).unwrap(), 2).unwrap();
    er.part2(oxygen_i * co2_i, Some(&format!("oxygen: {oxygen_i}, co2: {co2_i}")));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
".as_bytes()
        )
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 3 - binary diagnostic".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("198".to_string()));
        assert_eq!(er.answ()[1], Some("230".to_string()));
    }
}
