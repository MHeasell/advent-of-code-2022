use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("data/day1/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut largest = 0;
    let mut acc = 0;

    for line in lines {
        let line_contents = line.unwrap();
        if line_contents == "" {
            if acc > largest {
                largest = acc;
            }
            acc = 0;
        }
        else {
            acc += line_contents.parse::<i32>().unwrap();
        }
    }

    println!("Largest: {}", largest);
}
