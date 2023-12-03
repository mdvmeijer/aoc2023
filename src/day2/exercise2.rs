use std::path::Path;
use regex::Regex;
use crate::common::io;

pub fn run() {
    let input = io::read_file(Path::new("src/day2/input"));

    println!("Identifying the minimum sets of cubes...");
    let games = parse(input);

    let result: usize = games.iter().map(|game| {
        let red = dbg!(game.least_red_possible());
        let green = dbg!(game.least_green_possible());
        let blue = dbg!(game.least_blue_possible());
        red * green * blue
    })
        .sum();

    println!("Done. The sum of the power of the minimum set of cubes is {result}");
}

fn parse(input: String) -> Vec<Game> {
    let mut games = Vec::<Game>::new();
    for line in input.lines() {
        games.push(Game::new(line.to_owned()));
    }
    games
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

impl Game {
    fn new(line: String) -> Game {
        // TODO: don't compile Regex every time
        let r = Regex::new(r"(?:(\d+) (\w+),?)? ?(?:(\d+) (\w+),?)? ?(?:(\d+) (\w+))?").unwrap();
        let mut split = line.split(':');
        let game_prefix = split.next().unwrap();
        let mut game_prefix_split = game_prefix.split(' ');
        game_prefix_split.next();
        let game_id = game_prefix_split.next().unwrap().parse().unwrap();

        let sets_string = split.collect::<String>();
        let sets_string = sets_string.split(';');

        let mut sets = Vec::<Set>::new();
        for set_string in sets_string {
            let set_string = set_string.strip_prefix(' ').unwrap();
            sets.push(Set::new(set_string, &r));
        }
        Game { id: game_id, sets }
    }

    fn least_red_possible(&self) -> usize {
        self.sets.iter().map(|set| set.red).max().unwrap()
    }

    fn least_green_possible(&self) -> usize {
        self.sets.iter().map(|set| set.green).max().unwrap()
    }

    fn least_blue_possible(&self) -> usize {
        self.sets.iter().map(|set| set.blue).max().unwrap()
    }
}

impl Set {
    fn new(set_string: &str, r: &Regex) -> Set {
        let caps = r.captures(set_string).unwrap();

        let mut pairs: Vec<(usize, &str)> = Vec::new();

        let amount1 = caps.get(1).unwrap().as_str().parse().unwrap();
        let color1 = caps.get(2).unwrap().as_str();
        pairs.push((amount1, color1));

        let amount2 = caps.get(3);
        let color2 = caps.get(4);
        if let Some(amount) = amount2 { pairs.push((amount.as_str().parse().unwrap(), color2.unwrap().as_str())) }

        let amount3 = caps.get(5);
        let color3 = caps.get(6);
        if let Some(amount) = amount3 { pairs.push((amount.as_str().parse().unwrap(), color3.unwrap().as_str())) }

        let mut red: usize = 0;
        let mut green: usize = 0;
        let mut blue: usize = 0;
        for (amount, color) in pairs {
            if color == "red" { red = amount; }
            if color == "green" { green = amount; }
            if color == "blue" { blue = amount; }
        }

        Set {
            red,
            green,
            blue
        }
    }
}
