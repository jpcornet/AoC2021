use exrunner::ExRunner;
use std::{io, io::BufRead, collections::{HashMap, hash_map::Entry}};

#[derive(PartialEq, Debug, Clone, Copy)]
struct Board {
    numbers: [[u8; 5]; 5],
    // rows and cols counts the number of marked numbers in each row, col
    rows: [u8; 5],
    cols: [u8; 5],
    // cursor to implement an iterator over the numbers
    cursor: [u8; 2],
}

impl Board {
    fn parse(input: &str) -> Board {
        let mut numbers = [[0u8; 5]; 5];
        // make sure we have a proper number of rows
        let mut ok_rows = false;
        for (row, line) in input.split_terminator('\n').enumerate() {
            if row >= numbers.len() {
                panic!("Too many input rows for board in {line}");
            } else if row == numbers.len() - 1 {
                ok_rows = true;
            }
            let mut ok_cols = false;
            for (col, item) in line.split_whitespace().enumerate() {
                if col >= numbers[0].len() {
                    panic!("Too many items in input row in {line}");
                } else if col == numbers[0].len() - 1 {
                    ok_cols = true;
                }
                numbers[row][col] = item.parse().expect("Input should be numeric");
            }
            assert!(ok_cols, "Not enough columns");
        }
        assert!(ok_rows, "Not enough rows");
        Board {
            numbers,
            rows: [0; 5],
            cols: [0; 5],
            cursor: [0; 2],
        }
    }
}

// Iterate over all the numbers in the board
impl Iterator for Board {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor[1] as usize >= self.numbers.len() {
            return None;
        } else if self.cursor[0] as usize >= self.numbers[self.cursor[1] as usize].len() {
            self.cursor[0] = 0;
            self.cursor[1] += 1;
            if self.cursor[1] as usize >= self.numbers.len() {
                return None;
            }
        }
        let i: Self::Item = self.numbers[self.cursor[1] as usize][self.cursor[0] as usize];
        self.cursor[0] += 1;
        return Some(i);
    }
}

struct NumPos {
    board: usize,
    row: u8,
    col: u8,
}

struct PuzzleInput {
    draw: Vec<u8>,
    boards: Vec<Board>,
    numpos: HashMap<u8, Vec<NumPos>>,
}

impl PuzzleInput {
    fn parse(input: impl BufRead) -> PuzzleInput {
        let input = io::read_to_string(input).expect("Error reading input");
        let mut chunks = input.split("\n\n");
        let draw: Vec<u8> = chunks.next().expect("Input cannot be empty").split(',')
            .map(|x| x.parse().expect("Draw should be numbers")).collect();
        let boards: Vec<Board> = chunks.map(|c| Board::parse(c)).collect();
        let mut numpos: HashMap<u8, Vec<NumPos>> = HashMap::new();
        for (board, b) in boards.iter().enumerate() {
            for (row, r) in b.numbers.iter().enumerate() {
                for (col, i) in r.iter().enumerate() {
                    let e = numpos.entry(*i).or_insert(Vec::new());
                    e.push(NumPos{ board, row: row as u8, col: col as u8});
                }
            }
        }
        PuzzleInput { draw, boards, numpos }
    }
}

pub fn solve(input: impl BufRead, er: &mut ExRunner) {
    let mut pi = PuzzleInput::parse(input);
    er.parse_done();
    let mut num_drawn: HashMap<u8, ()> = HashMap::new();
    'draw: for d in pi.draw {
        // beware of duplicate numbers
        let num_entry = num_drawn.entry(d);
        if let Entry::Vacant(_) = num_entry {
            num_entry.or_insert(());
            if let Some(nps) = pi.numpos.get(&d) {
                for np in nps {
                    pi.boards[np.board].rows[np.row as usize] += 1;
                    pi.boards[np.board].cols[np.col as usize] += 1;
                    if pi.boards[np.board].rows[np.row as usize] == 5 || pi.boards[np.board].cols[np.col as usize] == 5 {
                        // We have a winner
                        // Assume no other simultaneous winner exists
                        println!("When drawing {d}, winning board is {}", np.board);
                        let unmarked_sum: u32 = pi.boards[np.board].filter_map(|i| if num_drawn.contains_key(&i) { None } else { Some(i as u32) }).sum();
                        er.part1(unmarked_sum*(d as u32), Some(&format!("unmarked_sum={unmarked_sum}, drawn={d}, board={}", np.board)));
                        break 'draw;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_board() {
        let b = Board::parse(
"22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19
");
       assert_eq!(b, Board { numbers: [ [ 22, 13, 17, 11, 0 ], [8, 2, 23, 4, 24], [21, 9, 14, 16, 7], [6, 10, 3, 18, 5], [1, 12, 20, 15, 19]], rows: [0; 5], cols: [0; 5], cursor: [0; 2]});
    }

    fn test_input() -> BufReader<&'static [u8]> {
        BufReader::new(
"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
".as_bytes()
        )
    }

    #[test]
    fn test_solve() {
        let er = ExRunner::run("day 4 - giant squid".to_string(), solve, test_input());
        er.print_raw();
        assert_eq!(er.answ()[0], Some("4512".to_string()));
        //assert_eq!(er.answ()[1], Some("".to_string()));
    }
}
