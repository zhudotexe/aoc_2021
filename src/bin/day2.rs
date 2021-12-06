use std::io::Read;

fn main() {
    // I figured out I/O!
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).expect("could not read input");
    println!("Part 1");
    part1(&buffer);
    println!("Part 2");
    part2(&buffer);
}

fn part1(input: &String) {
    let directions: Vec<&str> = input.split("\n").collect();
    let mut horiz = 0i32;
    let mut vert = 0i32;
    for direction in directions {
        match direction.split_once(" ") {
            Some(("forward", h)) => { horiz += h.parse::<i32>().expect("invalid forward") }
            Some(("down", v)) => { vert += v.parse::<i32>().expect("invalid down") }
            Some(("up", v)) => { vert -= v.parse::<i32>().expect("invalid up") }
            _ => panic!("invalid match")
        }
    }

    println!("Horiz: {} Vert: {}", horiz, vert);
    println!("Result: {}", horiz * vert)
}

fn part2(input: &String) {
    let directions: Vec<&str> = input.split("\n").collect();
    let mut horiz = 0i32;
    let mut vert = 0i32;
    let mut aim = 0i32;
    for direction in directions {
        let (op, amt) = direction.split_once(" ").expect("invalid instruction");
        match (op, amt.parse::<i32>().expect("invalid amount")) {
            ("forward", x) => {
                horiz += x;
                vert += aim * x
            }
            ("down", x) => { aim += x }
            ("up", x) => { aim -= x }
            _ => panic!("invalid match")
        }
    }
    // py3.10 equivalent because i got nerdsniped:
    /*
    directions = the_input.split("\n")
    for direction in directions:
      op, amt = direction.split(" ", 1)
      match op, int(amt):
        case "forward", x:
          horiz += x
          vert += aim * x
        case "down", x:
          aim += x
        case "up", x:
          aim -= x
        case _:
          raise ValueError
     */

    println!("Horiz: {} Vert: {}", horiz, vert);
    println!("Result: {}", horiz * vert)
}
