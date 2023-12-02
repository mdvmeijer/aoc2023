use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &Path) -> String {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut output_string = String::new();
    match file.read_to_string(&mut output_string) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => println!("File read successful")
    }
    output_string
}