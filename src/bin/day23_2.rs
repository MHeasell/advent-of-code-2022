use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};

fn main() {
    let file = File::open("data/day23/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());

    let mut state = parse_lines(lines);

    let mut i = 0;
    while state.moved {
        do_round(&mut state);
        i += 1;
    }

    println!("first static round: {}", i);
}

// fn print_grid(s: &HashSet<Position>) {
//     // let min_pos = s.iter().copied().reduce(|acc, p| Position{x:acc.x.min(p.x),y:acc.y.min(p.y)}).unwrap();
//     // let max_pos = s.iter().copied().reduce(|acc, p| Position{x:acc.x.max(p.x),y:acc.y.max(p.y)}).unwrap();
//     // let width = max_pos.x - min_pos.x + 1;
//     // let height = max_pos.y - min_pos.y + 1;

//     let x = 0;
//     let y = 0;
//     let width =5;
//     let height =6;

//     for dy in 0..height {
//         for dx in 0..width {
//             print!("{}", if s.contains(&Position{x:x+dx,y:y+dy}) { '#'} else {'.'})
//         }
//         println!()
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct State {
    elves: HashSet<Position>,
    first_dir: usize,
    moved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn try_get_proposal(state: &State, pos: Position, dir: Direction) -> Option<Position> {
    if !state.elves.contains(&Position{x:pos.x-1, y:pos.y-1})
        && !state.elves.contains(&Position{x:pos.x-1, y:pos.y})
        && !state.elves.contains(&Position{x:pos.x-1, y:pos.y+1})
        && !state.elves.contains(&Position{x:pos.x+1, y:pos.y-1})
        && !state.elves.contains(&Position{x:pos.x+1, y:pos.y})
        && !state.elves.contains(&Position{x:pos.x+1, y:pos.y+1})
        && !state.elves.contains(&Position{x:pos.x, y:pos.y-1})
        && !state.elves.contains(&Position{x:pos.x, y:pos.y+1}) {
        return None
    }
    match dir {
        Direction::North => {
            if !state.elves.contains(&Position{x:pos.x-1, y:pos.y-1})
                && !state.elves.contains(&Position{x:pos.x, y:pos.y-1})
                && !state.elves.contains(&Position{x:pos.x+1, y:pos.y-1}) {
                Some(Position{x:pos.x,y:pos.y-1})
            } else {None}
        }
        Direction::South => {
            if !state.elves.contains(&Position{x:pos.x-1, y:pos.y+1})
                && !state.elves.contains(&Position{x:pos.x, y:pos.y+1})
                && !state.elves.contains(&Position{x:pos.x+1, y:pos.y+1}) {
                Some(Position{x:pos.x,y:pos.y+1})
            } else { None }
        }
        Direction::West => {
            if !state.elves.contains(&Position{x:pos.x-1, y:pos.y-1})
                && !state.elves.contains(&Position{x:pos.x-1, y:pos.y})
                && !state.elves.contains(&Position{x:pos.x-1, y:pos.y+1}) {
                Some(Position{x:pos.x-1,y:pos.y})
            } else { None }
        }
        Direction::East => {
            if !state.elves.contains(&Position{x:pos.x+1, y:pos.y-1})
                && !state.elves.contains(&Position{x:pos.x+1, y:pos.y})
                && !state.elves.contains(&Position{x:pos.x+1, y:pos.y+1}) {
                Some(Position{x:pos.x+1,y:pos.y})
            } else { None }
        }
    }
}

fn get_proposal(state: &State, pos: Position) -> Option<Position> {
    let directions = [Direction::North, Direction::South, Direction::West, Direction::East];
    (0..directions.len()).find_map(|i| {
        let dir = directions[(i+state.first_dir)%directions.len()];
        try_get_proposal(state, pos, dir)
    })
}

fn propose_target(state: &State, pos: Position) -> Option<Position> {
    get_proposal(state, pos)
}

fn do_round(state: &mut State) {
    let mut target_squares = HashMap::new();
    let mut banned_squares = HashSet::new();

    for pos in &state.elves {
        if let Some(target) = propose_target(state, *pos) {
            if target_squares.insert(target, *pos).is_some() {
                banned_squares.insert(target);
            }
        }
    }

    state.moved = false;
    for (to, from) in &target_squares {
        if !banned_squares.contains(to) {
            state.elves.remove(from);
            state.elves.insert(*to);
            state.moved = true;
        }
    }

    state.first_dir = (state.first_dir + 1) % 4;
}

fn parse_lines<T: Iterator<Item=String>>(it: T) -> State {
    let mut hs = HashSet::new();
    for (y, l) in it.enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '#' => { hs.insert(Position{x:x.try_into().unwrap(),y: y.try_into().unwrap()}); }
                '.' => {}
                _ => panic!("unexpected char")
            }
        }
    }

    State { elves: hs, first_dir: 0, moved: true }
}