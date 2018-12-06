use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("1_input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
         Ok(_) => println!("read file"),
    }

    let mut split: Vec<&str> = s.split("\n").collect();
    println!("Array is length: {}", split.len());

    let mut still_looking = true;
    let mut total = 0;
    let mut totals = Vec::new();
    while still_looking {
        for value in split.iter() {
            if value == &"" {
                continue
            }

            let parsed: i32 = value.parse().unwrap();
            total += parsed;
            //println!("Parsing {}", parsed);
            //println!("Looking for {}", total);

            if totals.contains(&total) {
                println!("Found a duplicate!: {}", total);
                still_looking = false;
                break;
            }

            totals.push(total);
        }

        println!("Found no duplicates :( Gotta run through again!");
        println!("Total: {}", total);
        split = s.split("\n").collect();
    }

    // `file` goes out of scope, and the "1_input.txt" file gets closed
}
