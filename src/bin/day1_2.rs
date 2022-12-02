use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("data/day1/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut buf = vec![0, 0, 0];
    let mut acc = 0;

    for line in lines {
        let line_contents = line.unwrap();
        if line_contents == "" {
            let index = buf.iter().position(|x| x < &acc).unwrap_or(buf.len());
            buf.insert(index, acc);
            buf.pop();
            acc = 0;
        }
        else {
            acc += line_contents.parse::<i32>().unwrap();
        }
    }

    let total = buf.iter().sum::<i32>();

    println!("Buf: {:#?}", buf);
    println!("Total: {}", total);
}
