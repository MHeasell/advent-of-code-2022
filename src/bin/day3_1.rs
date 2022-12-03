use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let file = File::open("data/day3/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();


    let total = lines.into_iter().map(|l| compute_priority(&l.unwrap())).sum::<i32>();

    println!("Total: {}", total);
}

fn compute_priority(line: &str) -> i32 {
    if line.len() % 2 != 0 { panic!("line length not even"); }

    let split_idx = line.len() / 2;

    let (first_half, second_half) = line.split_at(split_idx);

    let common_elem = get_common_elem(first_half, second_half);

    common_elem.map(|e| get_elem_priority(e)).unwrap_or(0)
}

fn get_common_elem(first_half: &str, second_half: &str) -> Option<char> {
    let a = first_half.chars().collect::<HashSet<char>>();
    let b = second_half.chars().collect::<HashSet<char>>();

    let common_elems = a.intersection(&b).cloned().collect::<HashSet<char>>();

    match common_elems.len() {
        0 => None,
        1 => Some(common_elems.into_iter().next().unwrap()),
        _ => panic!("more than one common elem"),
    }
}

fn get_elem_priority(elem: char) -> i32 {
     match elem {
        'a'..='z' => ((elem as i32) - ('a' as i32)) + 1,
        'A'..='Z' => ((elem as i32) - ('A' as i32)) + 27,
        _ => panic!("char out of range: {}", elem),
    }
}
