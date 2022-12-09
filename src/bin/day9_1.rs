use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let file = File::open("data/day9/input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let lines_iter = lines.into_iter().map(|x| x.unwrap());

    let mut state = SimState{
        head_position: Position { x: 0, y: 0 },
        tail_position: Position { x: 0, y: 0 },
        tail_visited_cells: HashSet::from([Position{x:0, y:0}]),
    };

    for line in lines_iter {
        let instruction = decode_line(&line);
        for _ in 0..instruction.count {
            apply_step(&mut state, instruction.direction);
        }
    }

    println!("{}", &state.tail_visited_cells.len());
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

struct SimState {
    head_position: Position,
    tail_position: Position,
    tail_visited_cells: HashSet<Position>,
}

fn next_position(p: Position, d: Direction) -> Position {
    match d {
        Direction::Up => Position { x: p.x, y: p.y-1 },
        Direction::Down => Position { x: p.x, y: p.y+1 },
        Direction::Left => Position { x: p.x-1, y: p.y },
        Direction::Right => Position { x: p.x+1, y: p.y },
    }
}

fn pull_val(from: i32, to: i32) -> i32 {
    if to > from { return from + 1; }
    if to < from { return from - 1; }
    from
}

fn pull_pos(from: Position, to: Position) -> Position {
    Position{
        x: pull_val(from.x, to.x),
        y: pull_val(from.y, to.y),
    }
}

fn next_tail_position(tail_pos: Position, head_pos: Position) -> Position {
    if (head_pos.x - tail_pos.x).abs() >= 2 || (head_pos.y - tail_pos.y).abs() >= 2 {
        pull_pos(tail_pos, head_pos)
    } else {
        tail_pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_tail_position() {
        // on top of each other, don't move
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:0,y:0}), Position{x:0,y:0});

        // one space away, don't move
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:1,y:0}), Position{x:0,y:0});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:-1,y:0}), Position{x:0,y:0});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:0,y:1}), Position{x:0,y:0});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:0,y:-1}), Position{x:0,y:0});

        // two spaces away, move in one axis only
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:2,y:0}), Position{x:1,y:0});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:-2,y:0}), Position{x:-1,y:0});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:0,y:2}), Position{x:0,y:1});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:0,y:-2}), Position{x:0,y:-1});

        // two spaces plus diagonal
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:2,y:1}), Position{x:1,y:1});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:-2,y:1}), Position{x:-1,y:1});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:2,y:-1}), Position{x:1,y:-1});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:-2,y:-1}), Position{x:-1,y:-1});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:1,y:2}), Position{x:1,y:1});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:1,y:-2}), Position{x:1,y:-1});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:-1,y:2}), Position{x:-1,y:1});
        assert_eq!(next_tail_position(Position{x:0,y:0}, Position{x:-1,y:-2}), Position{x:-1,y:-1});
    }
}

fn apply_step(state: &mut SimState, direction: Direction) {
    state.head_position = next_position(state.head_position, direction);
    state.tail_position = next_tail_position(state.tail_position, state.head_position);
    state.tail_visited_cells.insert(state.tail_position);
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn char_to_direction(c: char) -> Direction {
    match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("invalid direction '{}'", c),
    }
}

struct Instruction {
    direction: Direction,
    count: i32,
}

fn decode_line(line: &str) -> Instruction {
    let mut iter = line.chars();
    let direction = char_to_direction(iter.next().unwrap());
    iter.next();
    let count = iter.as_str().parse::<i32>().unwrap();
    Instruction{ direction: direction, count: count }
}