use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Copy, Clone, Debug)]
struct Interval {
    first: i32,
    last: i32,
}

impl Interval {
    // fn contains(&self, other: &Interval) -> bool {
    //     self.first <= other.first && self.last >= other.last
    // }

    fn overlaps(&self, other: &Interval) -> bool {
        self.last >= other.first && self.first <= other.last
    }
}

fn main() {
    let file = File::open("data/day4/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let lines_iter = lines.into_iter();

    let count = lines_iter.map(|l| {
        let (a, b) = parse_line(&l.unwrap());
        if a.overlaps(&b) { println!("{:?}, {:?}", a, b); 1 } else { 0 }
    }).sum::<i32>();

    println!("Total: {}", count);
}

fn parse_line(line: &str) -> (Interval, Interval) {
    let parts = line.split(",").collect::<Vec<_>>();
    assert_eq!(2, parts.len());
    let intervals = parts.iter().map(|x| parse_interval(x)).collect::<Vec<_>>();
    (intervals[0], intervals[1])
}

fn parse_interval(s: &str) -> Interval {
    let parts = s.split("-").collect::<Vec<_>>();
    assert_eq!(2, parts.len());
    Interval {
        first: parts[0].parse().unwrap(),
        last: parts[1].parse().unwrap(),
    }
}
