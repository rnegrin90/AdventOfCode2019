use crate::helpers;

fn calculate_fuel(mass: i32) -> i32 {
    let mass_by_three = mass as f64 / 3 as f64;
    mass_by_three.floor() as i32 - 2
}

fn calculate_recursive_fuel(mass: i32) -> i32 {
    let mass_by_three = mass as f64 / 3 as f64;
    let mut fuel_required = mass_by_three.floor() as i32 - 2;
    if fuel_required > 0 {
        fuel_required += calculate_fuel(fuel_required);

        fuel_required
    } else {
        0
    }
}

pub fn solution(star: i8) -> i32 {
    if let Ok(lines) = helpers::read_lines("src/day01/input.txt") {
        let mut total_fuel = 0i32;
        for line in lines {
            if let Ok(number) = line {
                match star {
                    1 => total_fuel += calculate_fuel(number.parse::<i32>().unwrap()),
                    2 => total_fuel += calculate_recursive_fuel(number.parse::<i32>().unwrap()),
                    _ => ()
                }
            }
        }

        total_fuel
    } else {
        0i32
    }
}
