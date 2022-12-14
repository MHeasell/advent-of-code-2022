use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet, hash::Hash};

fn main() {
    let file = File::open("data/day14/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let paths = lines.map(|x| parse_path(&x.unwrap())).collect::<Vec<_>>();

    let max_y = paths.iter().flat_map(|p| p.iter().map(|pos| pos.y)).max().unwrap();
    let floor_y = max_y + 2;

    let mut obstacles = paths.iter().flat_map(|path| {
        path.windows(2).flat_map(|window| {
            assert_eq!(window.len(), 2);
            let s = window[0];
            let e = window[1];
            if s.x == e.x {
                let (start_y, end_y) = min_max(s.y, e.y);
                (start_y..=end_y).map(|y| Position{x:s.x, y}).collect::<Vec<_>>()
            } else if s.y == e.y {
                let (start_x, end_x) = min_max(s.x, e.x);
                (start_x..=end_x).map(|x| Position{x, y:s.y}).collect::<Vec<_>>()
            } else {
                panic!("diagonal line")
            }
        })
    }).collect::<HashSet<_>>();


    let mut count = 0;
    while add_sand(&mut obstacles, floor_y) {
        count += 1;
    }

    println!("count: {}", count);
}


/// Returns true if sand could be added
fn add_sand(obstacles: &mut HashSet<Position>, floor_y: usize) -> bool {
    let mut pos = Position{x: 500, y: 0};
    if obstacles.contains(&pos) { return false; }

    loop {
        let next_pos = next_sand_pos(obstacles, floor_y, pos);
        match next_pos {
            None => {
                obstacles.insert(pos);
                return true;
            }
            Some(p) => { pos = p; }
        }
    }
}

fn next_sand_pos(obstacles: &HashSet<Position>, floor_y: usize, pos: Position) -> Option<Position> {
    if pos.y == floor_y - 1 { return None }

    let candidate_positions = [
        Position{x: pos.x, y: pos.y+1},
        Position{x: pos.x-1, y: pos.y+1},
        Position{x: pos.x+1, y: pos.y+1},
    ];
    candidate_positions.into_iter().find(|p| !obstacles.contains(p))
}


fn min_max(a: usize, b: usize) -> (usize, usize) {
    if a < b { (a, b) } else { (b, a) }
}

fn parse_path(s: &str) -> Vec<Position> {
    s.split(" -> ").map(|item| {
        let parts = item.split(',').map(|n| n.parse().unwrap()).collect::<Vec<_>>();
        assert!(parts.len() == 2);
        Position{x: parts[0], y: parts[1]}
    }).collect()
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}
