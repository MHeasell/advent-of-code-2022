use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet, hash::Hash};

fn main() {
    let file = File::open("data/day18/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let positions = lines.map(|x| parse_position(&x.unwrap())).collect::<HashSet<_>>();

    let min_position = {
        let p = positions.iter().copied().reduce(|a, b| Position{x:a.x.min(b.x), y: a.y.min(b.y), z: a.z.min(b.z)}).unwrap();
        Position{x: p.x-1, y: p.y-1, z: p.z-1}
    };

    let max_position = {
        let p = positions.iter().copied().reduce(|a, b| Position{x:a.x.max(b.x), y: a.y.max(b.y), z: a.z.max(b.z)}).unwrap();
        Position{x: p.x+1, y: p.y+1, z: p.z+1}
    };

    let sum = flood_fill(positions, min_position, max_position);

    println!("{}", sum);
}

fn flood_fill(obstacles: HashSet<Position>, min_position: Position, max_position: Position) -> usize {
    let mut sum = 0usize;

    let mut visited = HashSet::<Position>::new();
    visited.insert(min_position);
    
    let mut open_list = vec![min_position];

    while let Some(position) = open_list.pop() {
        for neighbour in get_neighbours(&position) {
            if neighbour.x < min_position.x || neighbour.y < min_position.y || neighbour.z < min_position.z {
                continue;
            }
            if neighbour.x > max_position.x || neighbour.y > max_position.y || neighbour.z > max_position.z {
                continue;
            }
            if visited.contains(&neighbour) {
                continue;
            }
            if obstacles.contains(&neighbour) {
                sum += 1;
                continue;
            }

            open_list.push(neighbour);
            visited.insert(neighbour);
        }
    }

    sum
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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