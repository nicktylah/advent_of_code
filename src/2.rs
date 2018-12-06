mod files;
use std::collections::HashMap;

fn main() {
//    one();
    two();
}

fn one() {
    let lines = files::get_input("inputs/2_input.txt");

    let mut num_two_repeated_chars = 0;
    let mut num_three_repeated_chars = 0;
    for line in lines {
        let id = line.unwrap();
//        println!("{}", id);
        let mut chars = HashMap::new();
        for c in id.chars() {
//            println!("{}", c);
            // Takes a reference and returns Option<&V>
            match chars.get(&c) {
                Some(&count) => {
                    chars.insert(c, count + 1);
                },
                _ => {
                    chars.insert(c, 1);
                },
            }
        }

//        println!("{:?}", chars);
        let mut has_two = false;
        let mut has_three = false;
        for (c, &count) in chars.iter() {
            if count > 3 {
                println!("WTF, {} has a char: {} with {} occurrences", id, c, count);
                break
            }
            if count == 2 {
                has_two = true;
            }
            if count == 3 {
                has_three = true;
            }
        }

        if has_two {
            num_two_repeated_chars += 1;
        }
        if has_three {
            num_three_repeated_chars += 1;
        }
    }

    println!("Num w/ 2: {}\nNum w/ 3: {}\n Checksum: {}", num_two_repeated_chars, num_three_repeated_chars, num_two_repeated_chars*num_three_repeated_chars);
}

fn two() {
    let lines = files::get_input("inputs/2_input.txt");

    for line in lines {
        let id = line.unwrap();
//        println!("{}", id);
        let other_lines = files::get_input("inputs/2_input.txt");
        for other_line in other_lines {
            let other_id = other_line.unwrap();
            if id == other_id {
                continue
            }

            let mut num_diff = 0;
            for i in 0..id.len() {
                if id.chars().nth(i) == other_id.chars().nth(i) {
                    continue
                }
                num_diff += 1
            }

            if num_diff == 1 {
                println!("{} and {} are one character apart", id, other_id);
                println!("{}\n{}", id, other_id);
                break
            }
        }
    }
}
