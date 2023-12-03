use std::path::Path;
use regex::Regex;
use crate::common::io;

#[derive(Debug)]
struct Number {
    value: usize,
    x: usize,
    y: usize,
}

pub fn part1() {
    let input = io::read_file(&Path::new("src/day3/input"));
    let mut lines = Vec::<String>::new();

    for line in input.lines() {
        lines.push(line.to_string());
    }

    let mut numbers = Vec::<Number>::new();
    let number_regex = Regex::new(r"\d+").unwrap();

    for (line_index, line) in input.lines().enumerate() {
        let matches = number_regex.find_iter(line);
        for number_match in matches {
            println!("Match was found: {:?}", number_match);
            let number = dbg!(Number {
                value: number_match.as_str().parse().unwrap(),
                x: number_match.start(),
                y: line_index,
            });

            if dbg!(number.has_adjacent_symbol(&lines)) {
                numbers.push(number);
            }
        }
    }

    println!("The sum of all part numbers is {}", numbers.iter().map(|number| number.value).sum::<usize>());
}

impl Number {
    fn len(self: &Self) -> usize {
        self.value.to_string().len()
    }

    fn has_adjacent_symbol(self: &Self, lines: &Vec<String>) -> bool {
        let symbol_regex = Regex::new(r"[^a-zA-Z0-9.]").unwrap();
        let mut surrounding_chars_string = String::new();

        let at_top_edge = self.y == 0;
        let at_bottom_edge = self.y == lines.len() - 1;
        let at_left_edge = self.x == 0;
        let at_right_edge = self.x + self.len() - 1 == lines.get(0).unwrap().len() - 1;

        // Check line above
        if !at_top_edge {
            let line_above = lines.get(self.y - 1).unwrap();
            // diagonally left
            if !at_left_edge {
                surrounding_chars_string.push(line_above.chars().nth(self.x-1).unwrap())
            }
            // directly above
            for i in self.x..self.x + self.len() {
                surrounding_chars_string.push(line_above.chars().nth(i).unwrap())
            }
            // diagonally right
            if !at_right_edge {
                surrounding_chars_string.push(line_above.chars().nth(self.x + self.len()).unwrap())
            }
        }

        // Check same line
        let same_line = lines.get(self.y).unwrap();
        if !at_left_edge {
            surrounding_chars_string.push(same_line.chars().nth(self.x-1).unwrap())
        }
        if !at_right_edge {
            surrounding_chars_string.push(same_line.chars().nth(self.x + self.len()).unwrap())
        }

        // Check line below
        if !at_bottom_edge {
            let line_below = lines.get(self.y + 1).unwrap();
            // diagonally left
            if !at_left_edge {
                surrounding_chars_string.push(line_below.chars().nth(self.x-1).unwrap())
            }
            // directly below
            for i in self.x..self.x + self.len() {
                surrounding_chars_string.push(line_below.chars().nth(i).unwrap())
            }
            // diagonally right
            if !at_right_edge {
                surrounding_chars_string.push(line_below.chars().nth(self.x + self.len()).unwrap())
            }
        }

        if symbol_regex.is_match(surrounding_chars_string.as_str()) {
            true
        } else {
            false
        }
    }
}