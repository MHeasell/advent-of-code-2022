use std::{fs::File, io::{BufReader, BufRead}};

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

    let rocks = [ROCK_1, ROCK_2, ROCK_3, ROCK_4, ROCK_5].iter().map(|r| parse_rock(r)).collect::<Vec<_>>();

    let mut state = State {
        grid: Grid::new(7, 6, false),
        rocks: &rocks,
        rock_index: 0,
        pos: Position{x: 2, y: 3},
        directions: &directions,
        dir_index: 0,
        tower_height: 0,
        rocks_stopped: 0,
    };

    // print_state(&state);

    while state.rocks_stopped < 2022 {
    // for i in 0..20 {
        // println!("step {}", i);
        step(&mut state);
        // print_state(&state);
    }

    println!("height: {}", state.tower_height);
}

struct State<'a> {
    grid: Grid<bool>,
    rocks: &'a[Grid<bool>],
    rock_index: usize,
    pos: Position,
    directions: &'a[Direction],
    dir_index: usize,
    tower_height: usize,
    rocks_stopped: usize,
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
    x: i32,
    y: i32,
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

#[derive(Debug)]
struct Grid<T: Clone> {
    width: usize,
    vec: Vec<T>,
}

impl<T: Clone> Grid<T> {
    fn new(width: usize, height: usize, val: T) -> Self {

        Self { width: width, vec: vec![val; width*height] }
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

    fn iter(&self) -> impl Iterator<Item=(Position, &T)> {
        (0..self.height()).flat_map(|y| (0..self.width).map(move |x| Position{x: x.try_into().unwrap(),y: y.try_into().unwrap()})).map(|p| (p, self.get_pos(p)))
    }
}