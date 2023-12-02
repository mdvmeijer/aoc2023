use std::path::Path;
use std::collections::HashMap;
use crate::common::io;

pub fn run() {
    let input = io::read_file(&Path::new("src/day1/input"));

    let mut calibration_values = Vec::<u32>::new();

    for line in input.lines() {
        let (first_digit, first_digit_index) = get_first_digit(line);
        let (first_text_number, first_text_number_index) = get_first_text_number(line);

        let calibration_value_first_half = if first_digit_index < first_text_number_index {
            first_digit
        } else {
            first_text_number
        };

        let (last_digit, last_digit_index) = dbg!(get_last_digit(line));
        let (last_text_number, last_text_number_index) = dbg!(get_last_text_number(line));

        let calibration_value_second_half = if last_digit_index >= last_text_number_index {
            last_digit
        } else {
            last_text_number
        };

        calibration_values.push(format!("{calibration_value_first_half}{calibration_value_second_half}").parse::<u32>().unwrap());
    }

    println!("{}", calibration_values.iter().sum::<u32>());
}

fn get_first_digit(line: &str) -> (char, usize) {
    for (index, char) in line.chars().enumerate() {
        if char.is_digit(10) {
            return (char, index);
        }
    }

    panic!("did not find a digit")
}

fn get_first_text_number(line: &str) -> (char, usize) {
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
    let mut lowest_index = usize::MAX;
    let mut found_char = '`';

    for text_number in text_numbers.keys() {
        // find works with byte indices, so we assume all chars are 1 byte
        if let Some(index) = line.find(text_number) {
            if index < lowest_index {
                lowest_index = index;
                found_char = text_numbers.get(text_number).unwrap().to_owned();
            }
        }
    };

    (found_char, lowest_index)
}

fn get_last_digit(line: &str) -> (char, usize) {
    for (index, char) in line.chars().rev().enumerate() {
        if char.is_digit(10) {
            return (char, line.len() - index - 1);
        }
    }

    panic!("did not find a digit")
}

fn get_last_text_number(line: &str) -> (char, usize) {
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
    let mut lowest_index = usize::MAX;
    let mut found_text_number = "`";

    let line: String = line.chars().rev().collect();

    for text_number in text_numbers.keys() {
        let text_number_rev: String = text_number.chars().rev().collect();
        // find works with byte indices, so we assume all chars are 1 byte
        if let Some(index) = line.find(text_number_rev.as_str()) {
            if index < lowest_index {
                lowest_index = index;
                found_text_number = text_number;
            }
        }
    };

    let found_char = text_numbers.get(found_text_number).to_owned().unwrap_or(&'`');
    if found_text_number == "`" { (found_char.to_owned(), 0) } else { (found_char.to_owned(), line.len() - lowest_index - found_text_number.len()) }
}
