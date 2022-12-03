use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let file = File::open("data/day3/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();


    let mut total = 0;
    let mut lines_iter = lines.into_iter();

    loop {
        let a = lines_iter.next();
        if a.is_none() {
            break;
        }

        let b = lines_iter.next();
        let c = lines_iter.next();

        let elem = get_common_elem(&a.unwrap().unwrap(), &b.unwrap().unwrap(), &c.unwrap().unwrap());
        total += elem.map(|e| get_elem_priority(e)).unwrap_or(0);
    }

    println!("Total: {}", total);
}

fn get_common_elem(str_a: &str, str_b: &str, str_c: &str) -> Option<char> {
    let a = str_a.chars().collect::<HashSet<char>>();
    let b = str_b.chars().collect::<HashSet<char>>();
    let c = str_c.chars().collect::<HashSet<char>>();

    // Not the most elegant but it will do for now
    let common_elems = a.intersection(&b).cloned().collect::<HashSet<char>>().intersection(&c).cloned().collect::<HashSet<char>>();

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
