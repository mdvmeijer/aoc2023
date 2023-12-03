use std::path::Path;
use regex::Regex;
use crate::common::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Number {
    value: usize,
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct MatrixElement {
    value: char,
    x: usize,
    y: usize,
}

pub fn part2() {
    let binding = io::read_file(&Path::new("src/day3/input"));
    let mut input_lines: Vec<String> = binding
        .lines()
        .map(|str| String::from(str))
        .collect();

    println!("Identifying gears and calculating gear ratios...");
    let mut gear_candidate_to_numbers_map = HashMap::<MatrixElement, Vec<usize>>::new();
    let number_regex = Regex::new(r"\d+").unwrap();

    for (line_index, line) in input_lines.iter().enumerate() {
        let matches = number_regex.find_iter(line);
        for number_match in matches {
            let number = Number {
                value: number_match.as_str().parse().unwrap(),
                x: number_match.start(),
                y: line_index,
            };

            let gear_candidates = number.get_gear_candidates(&input_lines);
            for candidate in gear_candidates {
                let numbers = gear_candidate_to_numbers_map.entry(candidate).or_insert(Vec::new());
                numbers.push(number.value);
            }
        }
    }

    let result = gear_candidate_to_numbers_map.into_iter()
        .filter(|(_, numbers)| {
            numbers.len() == 2 // Two adjacent numbers, hence is a gear
        })
        .map(|(_, numbers)| {
            numbers.get(0).unwrap() * numbers.get(1).unwrap() // Calculate gear ratio
        })
        .sum::<usize>();

    println!("The sum of all gear ratios is {}", result);
}

pub fn part1() {
    let binding = io::read_file(&Path::new("src/day3/input"));
    let mut input_lines: Vec<String> = binding
        .lines()
        .map(|str| String::from(str))
        .collect();

    println!("Identifying part numbers and summing them...");
    let number_regex = Regex::new(r"\d+").unwrap();

    let numbers: Vec<Number> = input_lines
        .iter()
        .enumerate()
        .flat_map(|(line_index, line)| { // Create a Number for each match over all lines
            number_regex.find_iter(line).map(move |number_match| {
                Number {
                    value: number_match.as_str().parse().unwrap(),
                    x: number_match.start(),
                    y: line_index,
                }
            })
        })
        .filter(|number| number.has_adjacent_symbol(&input_lines))
        .collect();


    println!("The sum of all part numbers is {}", numbers.iter().map(|number| number.value).sum::<usize>());
}

impl MatrixElement {
    fn new(line: &String, x: usize, y: usize) -> MatrixElement {
        MatrixElement {
            value: line.chars().nth(x).unwrap(),
            x,
            y,
        }
    }
}

impl Number {
    fn len(self: &Self) -> usize {
        self.value.to_string().len()
    }

    fn get_gear_candidates(self: &Self, lines: &Vec<String>) -> Vec<MatrixElement> {
        let surrounding_matrix_elements = self.get_surrounding_matrix_elements(lines);

        surrounding_matrix_elements.into_iter()
            .filter(|element| element.value == '*')
            .collect()
    }

    fn get_surrounding_matrix_elements(self: &Self, lines: &Vec<String>) -> Vec<MatrixElement> {
        let mut surrounding_matrix_elements = Vec::<MatrixElement>::new();

        let at_top_edge = self.y == 0;
        let at_bottom_edge = self.y == lines.len() - 1;
        let at_left_edge = self.x == 0;
        let at_right_edge = self.x + self.len() - 1 == lines.get(0).unwrap().len() - 1;

        // Check line above
        if !at_top_edge {
            let line_above = lines.get(self.y - 1).unwrap();
            // diagonally left
            if !at_left_edge {
                let matrix_element = MatrixElement::new(&line_above, self.x-1, self.y-1);
                surrounding_matrix_elements.push(matrix_element);
            }
            // directly above
            for i in self.x..self.x + self.len() {
                let matrix_element = MatrixElement::new(&line_above, i, self.y-1);
                surrounding_matrix_elements.push(matrix_element);
            }
            // diagonally right
            if !at_right_edge {
                let matrix_element = MatrixElement::new(&line_above, self.x+self.len(), self.y-1);
                surrounding_matrix_elements.push(matrix_element);
            }
        }

        // Check same line
        let same_line = lines.get(self.y).unwrap();
        if !at_left_edge {
            let matrix_element = MatrixElement::new(&same_line, self.x-1, self.y);
            surrounding_matrix_elements.push(matrix_element);
        }
        if !at_right_edge {
            let matrix_element = MatrixElement::new(&same_line, self.x+self.len(), self.y);
            surrounding_matrix_elements.push(matrix_element);
        }

        // Check line below
        if !at_bottom_edge {
            let line_below = lines.get(self.y + 1).unwrap();
            if !at_left_edge {
                let matrix_element = MatrixElement::new(&line_below, self.x-1, self.y+1);
                surrounding_matrix_elements.push(matrix_element);
            }
            // directly below
            for i in self.x..self.x + self.len() {
                let matrix_element = MatrixElement::new(&line_below, i, self.y+1);
                surrounding_matrix_elements.push(matrix_element);
            }
            // diagonally right
            if !at_right_edge {
                let matrix_element = MatrixElement::new(&line_below, self.x+self.len(), self.y+1);
                surrounding_matrix_elements.push(matrix_element);
            }
        }

        surrounding_matrix_elements
    }

    fn has_adjacent_symbol(self: &Self, lines: &Vec<String>) -> bool {
        let symbol_regex = Regex::new(r"[^a-zA-Z0-9.]").unwrap();

        let surrounding_matrix_elements = self.get_surrounding_matrix_elements(lines);
        let surrounding_chars_string = surrounding_matrix_elements.iter()
            .map(|matrix_element| {
                matrix_element.value
            })
            .collect::<String>(); // Can collect Vec<char> as String

        if symbol_regex.is_match(surrounding_chars_string.as_str()) {
            true
        } else {
            false
        }
    }
}