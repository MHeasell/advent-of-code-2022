use std::{fs::File, io::{BufReader, BufRead}, collections::{VecDeque}};

const ROCK_1: &str = "####";
const ROCK_2: &str = ".#.\n###\n.#.";
const ROCK_3: &str = "..#\n..#\n###";
const ROCK_4: &str = "#\n#\n#\n#";
const ROCK_5: &str = "##\n##";

fn main() {
    let file = File::open("data/day17/input.txt").unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let directions = line.bytes().map(parse_direction).collect::<Vec<_>>();

    let rocks = create_initial_rocks();
    let mut state = create_initial_state(&directions, &rocks);

    let target_rocks_stopped = 1000000000000;

    let warmup_rocks_count = 100000;

    // warm it up
    for _ in 0..warmup_rocks_count {
        drop_rock(&mut state);
    }

    let initial_total_height = state.tower_height + state.discarded_rows;

    let cycle = find_cycle(&state);

    let total_height_gained_in_cycle = cycle.iter().sum::<usize>();
    let cycle_len = cycle.len();

    let total_cycles = (target_rocks_stopped - warmup_rocks_count) / cycle_len;

    let almost_final_height = initial_total_height + (total_cycles * total_height_gained_in_cycle);
    let current_rocks_stopped = warmup_rocks_count + (total_cycles * cycle_len);

    let remaining_height = (current_rocks_stopped..target_rocks_stopped).zip(cycle.iter().cycle()).map(|x| *x.1).sum::<usize>();

    let final_height = almost_final_height + remaining_height;

    println!("height: {}", final_height);
}

fn create_initial_rocks() -> Vec<Grid<bool>> {
    [ROCK_1, ROCK_2, ROCK_3, ROCK_4, ROCK_5].iter().map(|r| parse_rock(r)).collect::<Vec<_>>()
}

fn create_initial_state<'a>(directions: &'a [Direction], rocks: &'a [Grid<bool>]) -> State<'a> {
    State {
        grid: Grid::new(7, 6, false),
        rocks,
        rock_index: 0,
        pos: Position{x: 2, y: 3},
        directions,
        dir_index: 0,
        tower_height: 0,
        rocks_stopped: 0,
        discarded_rows: 0,
    }
}

fn create_delta_it(mut state: State) -> impl Iterator<Item=usize> + '_ {
    let initial_total_height = state.tower_height + state.discarded_rows;
    std::iter::repeat_with(move || {
        drop_rock(&mut state);
        state.tower_height + state.discarded_rows
    }).scan(initial_total_height, |st, x| {
        let delta = x - *st;
        *st = x;
        Some(delta)
    })
}

// Brute force find the cycle in the height increases.
// Pretty slow but it works.
fn find_cycle(initial_state: &State) -> Vec<usize> {
    for size in 1.. {
        let mut it = create_delta_it(initial_state.clone());
        let mut elems = Vec::new();
        for _ in 0..size {
            elems.push(it.next().unwrap());
        }
        if prove_cycle(&elems, &mut it) {
            return elems;
        }
    }

    panic!("should never get here")
}

// "proves" that the given iterator is a cycle of the given elems
// by iterating it a lot of times until we are convinced enough
fn prove_cycle<T: Iterator<Item=usize>>(elems: &[usize], it: &mut T) -> bool {
    it.zip(elems.iter().cycle().take(elems.len() * 1000)).all(|(x, y)| x == *y)
}

#[derive(Clone)]
struct State<'a> {
    grid: Grid<bool>,
    rocks: &'a[Grid<bool>],
    rock_index: usize,
    pos: Position,
    directions: &'a[Direction],
    dir_index: usize,
    tower_height: usize,
    rocks_stopped: usize,
    discarded_rows: usize,
}

fn apply_dir(p: Position, dir: Direction) -> Position {
    match dir {
        Direction::Left => Position{x:p.x-1, y: p.y},
        Direction::Right => Position{x:p.x+1, y: p.y},
    }
}

fn collides(grid: &Grid<bool>, pos: &Position, rock: &Grid<bool>) -> bool {
    if pos.x < 0 || pos.y < 0 || usize::try_from(pos.x).unwrap() + rock.width > grid.width || usize::try_from(pos.y).unwrap() + rock.height() > grid.height() {
        return true
    }

    rock.iter().map(|(p, v)| (p + *pos, v)).filter(|&(_, v)| *v).any(|(p, _)| *grid.get_pos(p))
}

fn stamp_down(grid: &mut Grid<bool>, pos: &Position, rock: &Grid<bool>) {
    for (p, v) in rock.iter().map(|(p, v)| (p + *pos, v)).filter(|&(_, v)| *v) {
        grid.set_pos(p, *v);
    }
}

/// really bad name, but returns true if blocks can't get through this row + prev row combined.
fn is_row_and_or_below_full(grid: &Grid<bool>, row: usize) -> bool {
    (0..grid.width).map(|x| {
        let v = *grid.get(x, row);
        let u = if row > 0 { *grid.get(x, row - 1) } else { true };
        v || u
    }).all(|v| v)
}

fn drop_rock(state: &mut State) {
    let rocks_stopped = state.rocks_stopped;
    while state.rocks_stopped == rocks_stopped {
        step(state);
    }
}

fn step(state: &mut State) {
    let rock = &state.rocks[state.rock_index];

    // left movement
    {
        let next_dir = state.directions[state.dir_index];
        state.dir_index = (state.dir_index + 1) % state.directions.len();
        let new_pos = apply_dir(state.pos, next_dir);
        if !collides(&state.grid, &new_pos, rock) {
            state.pos = new_pos;
        }
    }

    // down movement
    {
        let new_pos = Position{x:state.pos.x, y: state.pos.y-1};
        if collides(&state.grid, &new_pos, rock) {
            // set rock
            stamp_down(&mut state.grid, &state.pos, rock);
            state.rocks_stopped += 1;
            state.tower_height = state.tower_height.max(usize::try_from(state.pos.y).unwrap() + rock.height());

            // trim the grid down
            let trimmable_row_index = (0..rock.height())
                .rev()
                .map(|dy| usize::try_from(state.pos.y).unwrap() + dy)
                .find(|y| is_row_and_or_below_full(&state.grid, *y));
            if let Some(y) = trimmable_row_index {
                // We want to keep row y and drop all the rows below it.
                state.grid.drop_rows_from_start(y);
                state.tower_height -= y;
                state.discarded_rows += y;
            }

            // get a new rock
            state.rock_index = (state.rock_index + 1) % state.rocks.len();
            state.pos = Position{x: 2, y: (state.tower_height + 3).try_into().unwrap()};
            state.grid.resize_height(usize::try_from(state.pos.y).unwrap() + state.rocks[state.rock_index].height(), false);
        }
        else {
            state.pos = new_pos;
        }
    }
}

#[allow(dead_code)]
fn print_state(state: &State) {
    for y in (0..state.grid.height()).rev() {
        for x in 0..state.grid.width {
            let c =  { 
                let pos_in_rock = Position{x:x.try_into().unwrap(), y:y.try_into().unwrap()} - state.pos;
                let rock = &state.rocks[state.rock_index];
                match rock.try_get_pos(pos_in_rock) {
                    Some(true) => '@',
                    Some(false) | None =>  {
                        if *state.grid.get(x, y) { '#' } else { '.' }
                    }
                }
            };

            print!("{}", c);
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn parse_direction(dir: u8) -> Direction {
    match dir {
        b'<' => Direction::Left,
        b'>' => Direction::Right,
        _ => panic!("invalid direction"),
    }
}

fn parse_rock(s: &str) -> Grid<bool> {
    let lines = s.lines().collect::<Vec<_>>();
    let mut g = Grid::new(lines[0].len(), lines.len(), false);

    for (y, l) in lines.iter().rev().enumerate() {
        for (x, c) in l.bytes().enumerate() {
            let val = match c {
                b'#' => true,
                b'.' => false,
                _ => panic!("invalid char"),
            };
            g.set(x, y, val);
        }
    }

    g
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        Position{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

#[derive(Debug, Clone)]
struct Grid<T: Clone> {
    width: usize,
    vec: VecDeque<T>,
}

impl<T: Clone> Grid<T> {
    fn new(width: usize, height: usize, val: T) -> Self {
        let v = VecDeque::from_iter(std::iter::repeat(val).take(width*height));
        Self { width: width, vec: v }
    }

    fn height(&self) -> usize {
        self.vec.len() / self.width
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height()
    }

    fn pos_is_in_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && usize::try_from(pos.x).unwrap() < self.width && usize::try_from(pos.y).unwrap() < self.height()
    }

    fn get(&self, x: usize, y: usize) -> &T {
        assert!(self.is_in_bounds(x, y));
        &self.vec[(y*self.width)+x]
    }

    fn get_pos(&self, pos: Position) -> &T {
        self.get(pos.x.try_into().unwrap(), pos.y.try_into().unwrap())
    }

    #[allow(dead_code)]
    fn try_get(&self, x: usize, y: usize) -> Option<&T> {
        if self.is_in_bounds(x, y) {
            Some(self.get(x, y))
        }
        else {
            None
        }
    }

    fn try_get_pos(&self, pos: Position) -> Option<&T> {
        if self.pos_is_in_bounds(&pos) {
            Some(self.get_pos(pos))
        }
        else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        assert!(self.is_in_bounds(x, y));
        self.vec[(y*self.width)+x] = val;
    }

    fn set_pos(&mut self, pos: Position, val: T) {
        self.set(usize::try_from(pos.x).unwrap(), usize::try_from(pos.y).unwrap(), val);
    }

    fn resize_height(&mut self, rows: usize, val: T) {
        self.vec.resize(self.width * rows, val);
    }
    
    fn drop_rows_from_start(&mut self, rows: usize) {
        let new_start_idx = self.width * rows;
        self.vec.drain(0..new_start_idx);

    }

    fn iter(&self) -> impl Iterator<Item=(Position, &T)> {
        (0..self.height()).flat_map(|y| (0..self.width).map(move |x| Position{x: x.try_into().unwrap(),y: y.try_into().unwrap()})).map(|p| (p, self.get_pos(p)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_rock() {
        let directions_str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let directions = directions_str.bytes().map(parse_direction).collect::<Vec<_>>();
        let rocks = create_initial_rocks();
        let mut state = create_initial_state(&directions, &rocks);
        for _ in 0..2022 {
            drop_rock(&mut state);
        }
        assert_eq!(state.tower_height + state.discarded_rows, 3068);
    }
}
