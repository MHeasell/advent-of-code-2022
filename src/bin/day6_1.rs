use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("data/day6/sample_input.txt").unwrap();
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
    line.as_bytes().windows(4).enumerate().find(|(_, window)| is_unique(window)).map(|(i, _)| i+4)
}

fn is_unique(bytes: &[u8]) -> bool {
    for (i, a) in bytes.iter().enumerate() {
        for b in &bytes[i+1..] {
            if a == b { return false }
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

