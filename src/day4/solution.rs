use std::path::Path;
use crate::common::io;

pub fn part1() {
    let points: u32 = io::read_file(Path::new("src/day4/input"))
        .lines()
        .map(|line| {
            let mut line_split = line.split('|');
            let mut card_number_and_winning_numbers = line_split.next().unwrap().split(':');
            let _card_number = dbg!(card_number_and_winning_numbers.next().unwrap());
            let winning_numbers = card_number_and_winning_numbers
                .next()
                .unwrap()
                .split_whitespace()
                .map(|winning_number_str| winning_number_str.parse::<usize>().unwrap());
            let your_numbers = line_split.next()
                .unwrap()
                .split_whitespace()
                .map(|your_number_str| your_number_str.parse::<usize>().unwrap());
            (winning_numbers, your_numbers)
        })
        .map(|(winning_numbers, your_numbers)| {
            your_numbers
                .filter(|your_number| {
                    winning_numbers.clone().any(|winning_number| {
                        *your_number == winning_number
                    })
                })
                .count()
        })
        .map(|count| if count == 0 { 0 } else { 2u32.pow((count - 1) as u32) })
        .sum();
    println!("{points}")
}