use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, BufRead};
use std::fmt;

pub fn read_lines(name: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let path = Path::new(name);

    let file = match File::open(&path) {
        Err(why) => panic!(
            "Could not open {}: {}",
            path.display(),
            why.description()
        ),
        Ok(file)=> file
    };

    Ok(io::BufReader::new(file).lines())
}

pub fn read_file(name: &str) -> String {
    let path = Path::new(name);

    let mut file = match File::open(&path) {
        Err(why) => panic!(
            "Could not open {}: {}",
            path.display(),
            why.description()
        ),
        Ok(file)=> file
    };

    let mut result = String::new();

    match file.read_to_string(&mut result) {
        Err(why) => {
            panic!(
                "Could not open {}: {}",
                path.display(),
                why.description()
            )
        }
        Ok(_) => result
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn manhattan_distance(origin: Point, destination: Point) -> i32 {
    (origin.x - destination.x).abs() + (origin.y - destination.y).abs()
}
