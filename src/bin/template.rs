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
    unimplemented!()
}

fn part2(input: &String) {
    unimplemented!()
}