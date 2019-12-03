use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, BufRead};

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
