use crate::helpers::Point;

use std::fs::File;
use std::io::{self};

fn get_cable_instructions(raw_instructions: &String) -> Vec<(String, i32)> {
    raw_instructions
        .split(',')
        .map(
            |x| 
            (x[..1].to_string(), x[1..].parse::<i32>().unwrap())
        )
        .collect()
}

fn get_cable_segments(start: Point, instructions: Vec<(String, i32)>) -> Vec<(Point, Point)> {
    let mut current_point: Point = start;
    let mut asdf: Vec<(Point, Point)> = Vec::new();
    for instruction in instructions { 
        let new_point = match instruction.0.as_ref() {
            "U" => Point{x: current_point.x + instruction.1, y: current_point.y},
            "D" => Point{x: current_point.x - instruction.1, y: current_point.y},
            "L" => Point{x: current_point.x, y: current_point.y - instruction.1},
            "R" => Point{x: current_point.x, y: current_point.y + instruction.1},
            _ => panic!("This doesn't exist")
        };

        asdf.push((current_point, new_point));
        current_point = new_point;
    }

    asdf
}

pub fn solution(star: i8, mut lines: io::Lines<io::BufReader<File>>) -> i32 {
    let start_point = Point {
        x: 0i32,
        y: 0i32
    };

    let cable_1_segments: Vec<(Point, Point)>;
    if let Some(next_element) = lines.next() {
        match next_element {
            Err(why) => println!("nope"),
            Ok(cable_1_raw) => {
                let cable_1_instructions = get_cable_instructions(&cable_1_raw);
                cable_1_segments = get_cable_segments(start_point, cable_1_instructions);
            }
        }
    }

    let cable_2_segments: Vec<(Point, Point)>;
    if let Some(next_element) = lines.next() {
        match next_element {
            Err(why) => println!("nope"),
            Ok(cable_2_raw) => {
                let cable_2_instructions = get_cable_instructions(&cable_2_raw);
                cable_2_segments = get_cable_segments(start_point, cable_2_instructions);
            }
        }
    }

    0
}

#[cfg(test)]
mod tests;