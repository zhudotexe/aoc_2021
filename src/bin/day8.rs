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
    let (patterns, outputs): (Vec<&str>, Vec<&str>) = lines
        .iter()
        .map(|&line| line.split_once(" | ").expect("a"))
        .unzip();

    let mut result = 0;
    for output_line in outputs {
        for output in output_line.split(" ") {
            if output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7 {
                result += 1
            }
        }
    }
    println!("{}", result)
}

fn part2(input: &String) {
    unimplemented!()
}