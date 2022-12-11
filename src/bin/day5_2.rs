use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("data/day5/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut lines_iter = lines.into_iter().map(|x| x.unwrap());

    let rows = parse_stack(&mut lines_iter);

    println!("{:?}", rows);

    let mut stacks = vec![Vec::<char>::new(); rows[0].len()];
    for row in rows {
        for (i, elem) in row.iter().enumerate() {
            if *elem != ' ' {
                stacks[i].push(*elem);
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }

    println!("{:?}", stacks);

    assert_eq!(lines_iter.next().unwrap(), "");

    for instruction in lines_iter.map(|l| parse_instruction(&l)) {
        process_instruction(&mut stacks, &instruction);
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn process_instruction(stacks: &mut Vec<Vec<char>>, instruction: &Instruction) {
    // I was going to write a more efficient solution here, just directly copying,
    // but I ran into the classic "trying to mutably borrow 2 elements of a vec" issue.
    // I decided that just doing this less efficient "copy to buf and back" thing
    // would be less hassle.
    let mut buf = Vec::<char>::new();
    for _ in 0..instruction.count {
        let val = stacks[instruction.from-1].pop().unwrap();
        buf.push(val);
    }

    while !buf.is_empty() {
        stacks[instruction.to-1].push(buf.pop().unwrap());
    }
}

fn parse_stack<T: Iterator<Item=String>>(lines_iter: &mut T) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = Vec::new(); 

    for line in lines_iter.take_while(|l| l != "") {
        let mut row = Vec::<char>::new();
        let bytes = line.as_bytes();
        let mut i = 1;
        while i < bytes.len() {
            let b = bytes[i];
            if b == b'1' {
                return rows;
            }
            // Should be safe to just cast the byte to char
            // since we assume our input is just ASCII letters.
            row.push(b as char);
            i += 4;
        }
        rows.push(row);
    }

    panic!("shouldn't get here");
}

struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_instruction(line: &str) -> Instruction {
    let parts = line.split(' ').collect::<Vec<_>>();

    Instruction {
        from: parts[3].parse().unwrap(),
        to: parts[5].parse().unwrap(),
        count: parts[1].parse().unwrap()
    }
}