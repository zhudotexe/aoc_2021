use std::io::Read;
use memoise::memoise_map;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).expect("could not read input");
    println!("Part 1");
    part1(&buffer);
    println!("Part 2");
    part2(&buffer);
}

/// Given 1 lanternfish with a given *timer*, how many lanternfish will there be in *days* days?
#[memoise_map(timer, days)]
fn lanternfish(timer: u32, days: u32) -> u128 {
    match (timer, days) {
        (_, 0) => 1,
        (0, n) => lanternfish(6, n - 1) + lanternfish(8, n - 1),
        (t, d) => lanternfish(t - 1, d - 1)
    }
}

fn part1(input: &String) {
    let fish_timers: Vec<u32> = input
        .split(",")
        .map(|f| f.parse::<u32>().expect("invalid int"))
        .collect();
    let mut result = 0u128;
    for fish_timer in fish_timers {
        result += lanternfish(fish_timer, 80)
    }
    println!("Result: {}", result)
}

fn part2(input: &String) {
    let fish_timers: Vec<u32> = input
        .split(",")
        .map(|f| f.parse::<u32>().expect("invalid int"))
        .collect();
    let mut result = 0u128;
    for fish_timer in fish_timers {
        result += lanternfish(fish_timer, 256)
    }
    println!("Result: {}", result)
}