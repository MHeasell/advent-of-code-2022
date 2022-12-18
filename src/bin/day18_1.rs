use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let file = File::open("data/day18/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let positions = lines.map(|x| parse_position(&x.unwrap()));

    let mut m = HashSet::new();

    let mut sum = 0;

    for position in positions {
        sum += 6;
        for _ in get_neighbours(&position).iter().filter(|&p| m.contains(p)) {
            sum -= 2;
        }
        m.insert(position);
    }

    println!("{}", sum);
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i8,
    y: i8,
    z: i8
}

fn get_neighbours(p: &Position) -> [Position; 6] {
    [
        Position{x:p.x+1,y:p.y,z:p.z},
        Position{x:p.x-1,y:p.y,z:p.z},
        Position{x:p.x,y:p.y+1,z:p.z},
        Position{x:p.x,y:p.y-1,z:p.z},
        Position{x:p.x,y:p.y,z:p.z+1},
        Position{x:p.x,y:p.y,z:p.z-1},
    ]
}

fn parse_position(line: &str) -> Position {
    let parts = line.split(',').collect::<Vec<_>>();
    Position {
        x: parts[0].parse().unwrap(),
        y: parts[1].parse().unwrap(),
        z: parts[2].parse().unwrap(),
    }
}