use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::Lines;
use std::io::{BufRead, BufReader};

pub fn get_input(file_name: &str) -> Lines<std::io::BufReader<std::fs::File>> {
    // Create a path to the desired file
    let path = Path::new(file_name);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    return reader.lines();

    // Read the file contents into a string, returns `io::Result<usize>`
    /*let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => {
            println!("read file");
            let lines = &s.lines();
            return lines;
        }
    }*/

}
