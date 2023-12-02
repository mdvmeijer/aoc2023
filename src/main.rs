use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let path = Path::new("src/day1/input");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => println!("File read successful")
    }

    // println!("{input}");

    let mut calibration_values = Vec::<u32>::new();

    for line in input.lines() {
        let mut first_digit: char = 'a';
        let mut last_digit: char = 'a';
        for char in line.chars() {
            if char.is_digit(10) {
                first_digit = char;
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_digit(10) {
                last_digit = char;
                break;
            }
        }
        calibration_values.push(format!("{first_digit}{last_digit}").parse::<u32>().unwrap());
    }

    println!("{}", calibration_values.iter().sum::<u32>());
    // for x in calibration_values {
    //     println!("{x}")
    // }
}

// pub fn read_file(file: &Path) -> String {
//
// }
