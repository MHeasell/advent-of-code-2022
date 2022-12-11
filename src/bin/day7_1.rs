use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("data/day7/input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let mut stack = Vec::<i32>::new();

    let mut sum = 0;

    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        if line == "$ cd .." {
            let size = stack.pop().unwrap();
            *stack.last_mut().unwrap() += size;
            if size < 100000 {
                sum += size;
            }
            i += 1;
        }
        else if line.starts_with("$ cd ") {
            stack.push(0);
            i += 1;
        }
        else if line == "$ ls" {
            i += 1;
            while i < lines.len() {
                let l = &lines[i];
                if l.starts_with("$") {
                    break;
                }
                else if l.starts_with("dir ") {
                    i += 1;
                    continue;
                }
                else {
                    let mut parts = l.split(' ');
                    let size = parts.next().unwrap().parse::<i32>().unwrap();
                    *stack.last_mut().unwrap() += size;
                    i += 1;
                }
            }
        }
        else {
            panic!("unknown line: {}", line);
        }
    }

    println!("sum: {}", sum);
}
