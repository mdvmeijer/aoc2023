use std::path::Path;
use std::collections::HashMap;
use crate::common::io;

pub fn run() {
    let input = io::read_file(&Path::new("src/day1/input"));

    println!("Calculating calibration values...");
    let mut calibration_values = Vec::<u32>::new();

    for line in input.lines() {
        let (first_digit_index, first_digit) = get_first_digit(line);
        let (first_text_number_index, first_text_number) = get_first_text_number(line);

        let cal_val_first_digit = if first_digit_index < first_text_number_index {
            first_digit
        } else {
            first_text_number
        };

        let (last_digit_index, last_digit) = get_last_digit(line);
        let (last_text_number_index, last_text_number) = get_last_text_number(line);

        let cal_val_second_digit = if last_digit_index >= last_text_number_index {
            last_digit
        } else {
            last_text_number
        };

        let cal_val = format!("{cal_val_first_digit}{cal_val_second_digit}").parse::<u32>().unwrap();
        calibration_values.push(cal_val);
    }

    println!("Done. The sum of all calibration values is {}", calibration_values.iter().sum::<u32>());
}

fn get_first_digit(line: &str) -> (usize, char) {
    for (index, char) in line.chars().enumerate() {
        if char.is_digit(10) {
            return (index, char);
        }
    }

    panic!("did not find a digit") // TODO: handle with Result instead
}

fn get_first_text_number(line: &str) -> (usize, char) {
    let text_numbers = get_text_numbers_map();
    let mut lowest_index = usize::MAX;
    let mut found_char = '`'; // TODO: handle with Result instead

    for text_number in text_numbers.keys() {
        // find works with byte indices, so we assume all chars are 1 byte
        if let Some(index) = line.find(text_number) {
            if index < lowest_index {
                lowest_index = index;
                found_char = text_numbers.get(text_number).unwrap().to_owned();
            }
        }
    };

    (lowest_index, found_char)
}

fn get_last_digit(line: &str) -> (usize, char) {
    for (index, char) in line.chars().rev().enumerate() {
        if char.is_digit(10) {
            return (line.len() - index - 1, char);
        }
    }

    panic!("did not find a digit") // TODO: handle with Result instead
}

fn get_last_text_number(line: &str) -> (usize, char) {
    let text_numbers = get_text_numbers_map();

    let mut lowest_index = usize::MAX;
    let mut found_text_number = "`"; // TODO: handle with Result instead

    let line: String = line.chars().rev().collect();

    for text_number in text_numbers.keys() {
        let text_number_rev: String = text_number.chars().rev().collect();
        // `find` works with byte indices, so we assume all chars are 1 byte
        if let Some(index) = line.find(text_number_rev.as_str()) {
            if index < lowest_index {
                lowest_index = index;
                found_text_number = text_number;
            }
        }
    };

    let found_char = text_numbers.get(found_text_number).unwrap_or(&'`').to_owned();
    if found_text_number == "`" {
        (0, found_char)
    } else {
        (line.len() - lowest_index - found_text_number.len(), found_char)
    }
}

fn get_text_numbers_map() -> HashMap<&'static str, char> {
    let mut text_numbers = HashMap::<&str, char>::new();
    text_numbers.insert("one", '1');
    text_numbers.insert("two", '2');
    text_numbers.insert("three", '3');
    text_numbers.insert("four", '4');
    text_numbers.insert("five", '5');
    text_numbers.insert("six", '6');
    text_numbers.insert("seven", '7');
    text_numbers.insert("eight", '8');
    text_numbers.insert("nine", '9');

    text_numbers
}