use std::path::Path;
use crate::common::io;

pub fn run() {
    let input = io::read_file(&Path::new("src/day1/input"));

    println!("Calculating calibration values...");
    let mut calibration_values = Vec::<u32>::new();

    for line in input.lines() {
        let mut first_digit: char = 'a'; // TODO: Find better way of managing no-digit-found
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

    println!("Done. The sum of all calibration values is {}", calibration_values.iter().sum::<u32>());
}