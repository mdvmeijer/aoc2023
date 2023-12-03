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
    y: usize
}

pub fn part2() {
    let input = io::read_file(&Path::new("src/day3/input"));
    let mut lines = Vec::<String>::new();

    for line in input.lines() {
        lines.push(line.to_string());
    }

    let mut gear_candidate_to_number_map = HashMap::<MatrixElement, Vec<usize>>::new();
    let number_regex = Regex::new(r"\d+").unwrap();

    for (line_index, line) in input.lines().enumerate() {
        let matches = number_regex.find_iter(line);
        for number_match in matches {
            let number = dbg!(Number {
                value: number_match.as_str().parse().unwrap(),
                x: number_match.start(),
                y: line_index,
            });

            let gear_candidates = dbg!(number.get_gear_candidates(&lines));
            for candidate in gear_candidates {
                let entry = gear_candidate_to_number_map.entry(candidate).or_insert(Vec::new());
                entry.push(number.value);
            }
        }
    }

    let gear_to_number_map: HashMap<MatrixElement, Vec<usize>> = gear_candidate_to_number_map.into_iter().filter(|(_, numbers)| numbers.len() == 2 ).collect();

    let result = gear_to_number_map.iter().map(|(gear, numbers)| {
        numbers.get(0).unwrap() * numbers.get(1).unwrap()
    }).sum::<usize>();

    println!("The sum of all gear ratios is {}", result);
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

    fn get_gear_candidates(self: &Self, lines: &Vec<String>) -> Vec<MatrixElement> {
        let matrix_elements = self.get_surrounding_matrix_elements(lines);

        let gear_candidates = matrix_elements.into_iter().filter(|element| element.value == '*').collect();
        gear_candidates
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
                let matrix_element = MatrixElement {
                    value: line_above.chars().nth(self.x-1).unwrap(),
                    x: self.x-1,
                    y: self.y-1
                };
                surrounding_matrix_elements.push(matrix_element);
            }
            // directly above
            for i in self.x..self.x + self.len() {
                let matrix_element = MatrixElement {
                    value: line_above.chars().nth(i).unwrap(),
                    x: i,
                    y: self.y-1
                };
                surrounding_matrix_elements.push(matrix_element);
            }
            // diagonally right
            if !at_right_edge {
                let matrix_element = MatrixElement {
                    value: line_above.chars().nth(self.x + self.len()).unwrap(),
                    x: self.x + self.len(),
                    y: self.y-1
                };
                surrounding_matrix_elements.push(matrix_element);
            }
        }

        // Check same line
        let same_line = lines.get(self.y).unwrap();
        if !at_left_edge {
            let matrix_element = MatrixElement {
                value: same_line.chars().nth(self.x-1).unwrap(),
                x: self.x-1,
                y: self.y
            };
            surrounding_matrix_elements.push(matrix_element);
        }
        if !at_right_edge {
            let matrix_element = MatrixElement {
                value: same_line.chars().nth(self.x + self.len()).unwrap(),
                x: self.x + self.len(),
                y: self.y
            };
            surrounding_matrix_elements.push(matrix_element);
        }

        // Check line below
        if !at_bottom_edge {
            let line_below = lines.get(self.y + 1).unwrap();
            if !at_left_edge {
                let matrix_element = MatrixElement {
                    value: line_below.chars().nth(self.x-1).unwrap(),
                    x: self.x-1,
                    y: self.y+1
                };
                surrounding_matrix_elements.push(matrix_element);
            }
            // directly below
            for i in self.x..self.x + self.len() {
                let matrix_element = MatrixElement {
                    value: line_below.chars().nth(i).unwrap(),
                    x: i,
                    y: self.y+1
                };
                surrounding_matrix_elements.push(matrix_element);
            }
            // diagonally right
            if !at_right_edge {
                let matrix_element = MatrixElement {
                    value: line_below.chars().nth(self.x + self.len()).unwrap(),
                    x: self.x + self.len(),
                    y: self.y+1
                };
                surrounding_matrix_elements.push(matrix_element);
            }
        }

        surrounding_matrix_elements
    }

    fn has_adjacent_symbol(self: &Self, lines: &Vec<String>) -> bool {
        let symbol_regex = Regex::new(r"[^a-zA-Z0-9.]").unwrap();

        let surrounding_matrix_elements = self.get_surrounding_matrix_elements(lines);
        let surrounding_chars_string = surrounding_matrix_elements.iter().map(|matrix_element| {
            matrix_element.value
        }).collect::<String>(); // Can collect Vec<char> as String

        if symbol_regex.is_match(surrounding_chars_string.as_str()) {
            true
        } else {
            false
        }
    }
}