use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("data/day7/input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let mut stack = Vec::<i32>::new();

    let mut final_sizes = Vec::<i32>::new();

    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        if line == "$ cd .." {
            let size = stack.pop().unwrap();
            *stack.last_mut().unwrap() += size;
            final_sizes.push(size);
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

    // Flush the remaining stuff out of the stack.
    // Didn't do this in part 1 solution, whoops.
    // Somehow got lucky?
    while stack.len() > 1 {
        let size = stack.pop().unwrap();
        *stack.last_mut().unwrap() += size;
        final_sizes.push(size);
    }
    final_sizes.push(stack.pop().unwrap());
    

    println!("{:?}", final_sizes);

    let total_space = 70000000;
    let required_space = 30000000;
    let used_space = final_sizes.last().unwrap();
    let free_space = total_space - used_space;
    let additional_space_needed = required_space - free_space;

    println!("used space: {}", used_space);
    println!("free space: {}", free_space);
    println!("additional space needed: {}", additional_space_needed);

    let candidate_dir_size = final_sizes
        .iter()
        .filter(|size| **size >= additional_space_needed)
        .min_by_key(|size| **size)
        .unwrap();
    
    println!("{}", candidate_dir_size);
}
