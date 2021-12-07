use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).expect("could not read input");
    println!("Part 1");
    part1(&buffer);
    println!("Part 2");
    part2(&buffer);
}

// === common ===
#[derive(Debug)]
struct BingoEntry {
    n: u8,
    marked: bool,
}

#[derive(Debug)]
struct BingoBoard {
    entries: Vec<BingoEntry>, // flat 5x5
}

impl BingoBoard {
    fn new(numbers: &Vec<u8>) -> BingoBoard {
        let mut entries: Vec<BingoEntry> = Vec::new();
        for &number in numbers.iter().take(25) {
            entries.push(BingoEntry { n: number, marked: false })
        }
        BingoBoard { entries }
    }

    fn is_bingo(&self) -> bool {
        // row
        for row in 0..4 {
            if self.entries.iter().skip(row * 5).take(5).all(|entry| entry.marked) {
                return true;
            }
        }
        // col
        for col in 0..4 {
            if self.entries.iter().skip(col).step_by(5).all(|entry| entry.marked) {
                return true;
            }
        }
        false
    }

    fn mark(&mut self, n: u8) {
        for entry in self.entries.iter_mut() {
            if entry.n == n {
                entry.marked = true;
            }
        }
    }

    fn unmarked(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for entry in self.entries.iter() {
            if !entry.marked { result.push(entry.n) }
        }
        result
    }
}

fn parse_input(input: &String) -> (Vec<u8>, Vec<BingoBoard>) {
    let draws: Vec<u8> = input
        .split("\n")
        .next()
        .unwrap_or("")
        .split(",")
        .map(|n| n.parse::<u8>().expect("invalid int"))
        .collect();
    let boards: Vec<BingoBoard> = input
        .split("\n\n")
        .skip(1)
        .map(|board_str| {
            let board_nums = board_str
                .split_whitespace()
                .map(|n| n.parse::<u8>().expect("invalid int"))
                .collect();
            BingoBoard::new(&board_nums)
        })
        .collect();
    (draws, boards)
}

// === p1 ===
fn part1(input: &String) {
    let (draws, mut boards) = parse_input(input);
    'outer:
    for draw in draws {
        for board in boards.iter_mut() {
            board.mark(draw);
            if board.is_bingo() {
                let unmarked_sum: u32 = board.unmarked().iter().fold(0, |acc, &n| acc + n as u32);
                println!("BINGO! Solution: {}", unmarked_sum * (draw as u32));
                break 'outer;
            }
        }
    }
}


// === p2 ===
fn part2(input: &String) {
    let (draws, mut boards) = parse_input(input);
    let mut winning_boards: Vec<usize> = Vec::new();
    for draw in draws {
        for (idx, board) in boards.iter_mut().enumerate() {
            board.mark(draw);
            if board.is_bingo() && !winning_boards.contains(&idx) {
                winning_boards.push(idx);
                let unmarked_sum: u32 = board.unmarked().iter().fold(0, |acc, &n| acc + n as u32);
                println!("BINGO! Solution: {}", unmarked_sum * (draw as u32));
            }
        }
    }
}