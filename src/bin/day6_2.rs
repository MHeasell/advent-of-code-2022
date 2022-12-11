use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let file = File::open("data/day6/input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let lines_iter = lines.into_iter().map(|x| x.unwrap());

    lines_iter.for_each(|l| {
        println!("line: {}", l);
        let idx = find_idx(&l);
        match idx {
            Some(idx) => println!("{}", idx),
            None => println!("No elem"),
        }
    });
}

fn find_idx(line: &str) -> Option<usize> {
    line.as_bytes().windows(14).enumerate().find(|(_, window)| is_unique(window)).map(|(i, window)| i+window.len())
}

fn is_unique(bytes: &[u8]) -> bool {
    let mut s = HashSet::<u8>::new();
    for b in bytes {
        if !s.insert(*b) {
            return false
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_unique() {
        assert_eq!(is_unique(&[1,2,3,4]), true);
        assert_eq!(is_unique(&[1,1,3,4]), false);
        assert_eq!(is_unique(&[1,2,1,4]), false);
        assert_eq!(is_unique(&[1,2,3,1]), false);
    }
}

