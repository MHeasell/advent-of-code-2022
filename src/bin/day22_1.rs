use std::{fs::File, io::{BufReader, BufRead}, iter::Peekable};

fn main() {
    let file = File::open("data/day22/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let sep = lines.iter().position(|l| l.is_empty()).unwrap();

    let grid = parse_grid(&lines[0..sep]);
    let instructions = parse_instructions(&lines[sep+1]);

    let mut state = State {
        pos: Position {x:0,y:0},
        dir: Direction::Right,
    };

    for instruction in instructions {
        state = apply_instruction(&grid, &state, instruction);
    }

    let password = get_password(&state);

    println!("password: {}", password);
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
struct State {
    pos: Position,
    dir: Direction,
}

fn get_password(s: &State) -> i64 {
    let row = s.pos.y + 1;
    let col = s.pos.x + 1;
    let facing = match s.dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    (1000 * row) + (4 * col) + facing
}

fn get_next_pos(pos: Position, dir: Direction) -> Position {
    match dir {
        Direction::Up => Position{x:pos.x,y:pos.y-1},
        Direction::Down => Position{x:pos.x,y:pos.y+1},
        Direction::Right => Position{x:pos.x+1,y:pos.y},
        Direction::Left => Position{x:pos.x-1,y:pos.y},
    }
}

fn wrap_pos<T: Clone>(grid: &Grid<T>, pos: Position) -> Position {
    Position {
        x: wrap_val(pos.x, grid.width.try_into().unwrap()),
        y: wrap_val(pos.y, grid.height().try_into().unwrap()),
    }
}

fn get_next_pos_wrapped(grid: &Grid<TerrainType>, pos: Position, dir: Direction) -> Position {
    let new_pos_raw = get_next_pos(pos, dir);
    wrap_pos(grid, new_pos_raw)
}

fn wrap_val(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

fn go_forward_one_step(grid: &Grid<TerrainType>, state: &State) -> Option<State> {
    let mut new_pos = get_next_pos_wrapped(grid, state.pos, state.dir);
    loop {
        match grid.get_pos(new_pos) {
            TerrainType::Floor => { return Some(State{pos:new_pos,dir:state.dir}); }
            TerrainType::Wall => { return None; },
            TerrainType::OutOfBounds => { new_pos = get_next_pos_wrapped(grid, new_pos, state.dir); },
        }
    }
}

fn rotate_dir_left(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
        Direction::Left => Direction::Down,
    }
}

fn rotate_dir_right(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
        Direction::Left => Direction::Up,
    }
}

fn turn_left(state: &State) -> State {
    State {
        pos: state.pos,
        dir: rotate_dir_left(state.dir),
    }
}

fn turn_right(state: &State) -> State {
    State {
        pos: state.pos,
        dir: rotate_dir_right(state.dir),
    }
}

fn go_forward(grid: &Grid<TerrainType>, state: &State, steps: i32) -> State {
    let mut s = *state;
    for _ in 0..steps {
        match go_forward_one_step(grid, &s) {
            Some(new_s) => { s = new_s; }
            None => { return s; }
        }
    }

    s
}

fn apply_instruction(grid: &Grid<TerrainType>, state: &State, instruction: Instruction) -> State {
    match instruction {
        Instruction::Forward(steps) => go_forward(grid, state, steps),
        Instruction::TurnLeft => turn_left(state),
        Instruction::TurnRight => turn_right(state),
    }
}

fn parse_grid(lines: &[String]) -> Grid<TerrainType> {
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).reduce(std::cmp::max).unwrap();
    let mut g = Grid::new(width, height, TerrainType::OutOfBounds);

    for (y, line) in lines.iter().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            match b {
                b'.' => { g.set(x, y, TerrainType::Floor); }
                b'#' => { g.set(x, y, TerrainType::Wall); }
                _ => {}
            }
        }
    }

    g
}

fn parse_number<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> i32 {
    let mut buf = String::new();
    while let Some(c) = iter.peek() {
        if !c.is_ascii_digit() {
            break;
        }
        buf.push(*c);
        iter.next();
    }
    buf.parse().unwrap()
}

fn parse_turn<T: Iterator<Item=char>>(iter: &mut T) -> Instruction {
    let c = iter.next().unwrap();
    match c {
        'L' => Instruction::TurnLeft,
        'R' => Instruction::TurnRight,
        _ => panic!("invalid input")
    }
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut v = Vec::new();
    let mut it = line.chars().peekable();
    loop {
        match it.peek() {
            Some(x) if x.is_ascii_digit() => v.push(Instruction::Forward(parse_number(&mut it))),
            Some(_) => v.push(parse_turn(&mut it)),
            None => break
        }
    }
    v
}

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TerrainType {
    Wall,
    Floor,
    OutOfBounds,
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
    vec: Vec<T>,
}

impl<T: Clone> Grid<T> {
    fn new(width: usize, height: usize, val: T) -> Self {
        let v = vec![val; width*height];
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

    fn iter(&self) -> impl Iterator<Item=(Position, &T)> {
        (0..self.height()).flat_map(|y| (0..self.width).map(move |x| Position{x: x.try_into().unwrap(),y: y.try_into().unwrap()})).map(|p| (p, self.get_pos(p)))
    }
}