mod helpers;

mod day01;
mod day02;

fn main() {
    // Day 01

    if let Ok(lines) = helpers::read_lines("src/day01/input.txt") {
        print!("Day01 - Solution 1: {}", day01::solution(1i8, lines));
    } else {
        panic!("Could not read file for day 1!");
    }

    if let Ok(lines) = helpers::read_lines("src/day01/input.txt") {
        println!(" - Solution 2: {}", day01::solution(2i8, lines));
    } else {
        panic!("Could not read file for day 1!");
    }

    // Day 02

    let input = helpers::read_file("src/day02/input.txt");

    print!("Day02 - Solution 1: {}", day02::solution(1i8, &input));
    println!(" - Solution 2: {}", day02::solution(2i8, &input));
}
