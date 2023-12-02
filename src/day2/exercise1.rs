use std::path::Path;
use crate::common::io;

pub fn run() {
    let input = io::read_file(&Path::new("src/day2/input"));

    let set1 = Set { red: 4, green: 0, blue: 3 };
    let set2 = Set { red: 1, green: 2, blue: 6 };
    let set3 = Set { red: 0, green: 2, blue: 0 };
    let game = Game { sets: vec![set1, set2, set3] };

    dbg!(game.is_possible(
        499, 9, 6
    ));
}

struct Game {
    sets: Vec<Set>
}

struct Set {
    red: usize,
    green: usize,
    blue: usize
}

impl Game {
    fn is_possible(
        self: &Self,
        red_max: usize,
        green_max: usize,
        blue_max: usize
    ) -> bool {
        if self.sets.iter().any(|set| set.red > red_max) { return false; }
        if self.sets.iter().any(|set| set.green > green_max) { return false; }
        if self.sets.iter().any(|set| set.blue > blue_max) { return false; }
        true
    }
}
