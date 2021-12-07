use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).expect("could not read input");
    println!("Part 1");
    part1(&buffer);
    println!("Part 2");
    part2(&buffer);
}

fn part1(input: &String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let line_len = lines[0].len();
    let num_lines = lines.len();
    let mask = !(u32::MAX << line_len);  // trickery: lower (line_len) bits are 1
    let mut counts: Vec<u32> = vec![0; line_len];

    // get counts of 1s in each col
    for line in lines {
        for (idx, char) in line.chars().enumerate() {
            match char {
                '0' => {}
                '1' => { counts[idx] += 1 }
                _ => panic!("invalid char")
            }
        }
    }

    // unpack those counts into an int
    let mut gamma = 0u32;
    for count in counts.iter() {
        gamma <<= 1;
        if count * 2 > num_lines as u32 {
            gamma += 1
        }
    }
    let epsilon = !gamma & mask;

    println!("gamma: {0:0width$b} epsilon: {1:0width$b}", gamma, epsilon, width = line_len);
    println!("result: {}", gamma * epsilon)
}

// === part 2 ===
// other fns defined below were not used in p1
fn part2(input: &String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let line_len = lines[0].len();

    let mut col = 0u8;
    let mut o2_lines = lines.to_vec();
    while o2_lines.len() > 1 {
        match most_common_in_col(&o2_lines, col) {
            CommonVal::Zero => { filter_from_col(&mut o2_lines, col, '0') }
            CommonVal::One => { filter_from_col(&mut o2_lines, col, '1') }
            CommonVal::Equal => { filter_from_col(&mut o2_lines, col, '1') }
        }
        col += 1;
    }
    let o2_rating = u32::from_str_radix(o2_lines[0], 2).expect("invalid number in o2 rating");

    let mut col = 0u8;
    let mut co2_lines = lines.to_vec();
    while co2_lines.len() > 1 {
        match most_common_in_col(&co2_lines, col) {
            CommonVal::Zero => { filter_from_col(&mut co2_lines, col, '1') }
            CommonVal::One => { filter_from_col(&mut co2_lines, col, '0') }
            CommonVal::Equal => { filter_from_col(&mut co2_lines, col, '0') }
        }
        col += 1;
    }
    let co2_rating = u32::from_str_radix(co2_lines[0], 2).expect("invalid number in co2 rating");


    println!("o2: {0:0width$b} co2: {1:0width$b}", o2_rating, co2_rating, width = line_len);
    println!("result: {}", o2_rating * co2_rating)
}

enum CommonVal {
    Zero = 0,
    One = 1,
    Equal,
}

fn most_common_in_col(lines: &Vec<&str>, col: u8) -> CommonVal {
    let num_lines = lines.len();
    let mut count = 0u32;
    for line in lines {
        match line.chars().nth(col as usize).expect("out of bounds") {
            '0' => {}
            '1' => { count += 1 }
            _ => panic!("invalid char")
        }
    }
    if count * 2 > num_lines as u32 {
        CommonVal::One
    } else if count * 2 == num_lines as u32 {
        CommonVal::Equal
    } else {
        CommonVal::Zero
    }
}

fn filter_from_col(lines: &mut Vec<&str>, col: u8, val: char) {
    lines.retain(|&line| line.chars().nth(col as usize).expect("out of bounds") == val)
}
