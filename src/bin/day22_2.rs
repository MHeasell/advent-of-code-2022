use std::{fs::File, io::{BufReader, BufRead}, iter::Peekable, collections::HashMap};

// const FACE_SIZE: usize = 4;
const FACE_SIZE: usize = 50;

// const STARTING_FACE: i64 = 0;
const STARTING_FACE: i64 = 1;

fn main() {
    // let file = File::open("data/day22/sample_input.txt").unwrap();
    let file = File::open("data/day22/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let sep = lines.iter().position(|l| l.is_empty()).unwrap();

    let grid = parse_grid(&lines[0..sep]);

    // let cube = create_cube_from_sample_input(&grid);
    let cube = create_cube_from_input(&grid);

    let instructions = parse_instructions(&lines[sep+1]);

    let mut state = State {
        pos: CubePosition {face: STARTING_FACE, pos: Position{x:0,y:0}},
        dir: Direction::Right,
    };

    for instruction in instructions {
        state = apply_instruction(&cube, &state, instruction);
    }

    let password = get_password(&state);

    println!("password: {}", password);
}

fn create_cube_from_input(grid: &Grid<TerrainType>) -> Cube {
    let face0 = grid.extract(FACE_SIZE*2, 0,           FACE_SIZE, FACE_SIZE);
    let face1 = grid.extract(FACE_SIZE,   0,           FACE_SIZE, FACE_SIZE);
    let face2 = grid.extract(FACE_SIZE,   FACE_SIZE,   FACE_SIZE, FACE_SIZE);
    let face3 = grid.extract(FACE_SIZE,   FACE_SIZE*2, FACE_SIZE, FACE_SIZE);
    let face4 = grid.extract(0,           FACE_SIZE*2, FACE_SIZE, FACE_SIZE);
    let face5 = grid.extract(0,           FACE_SIZE*3, FACE_SIZE, FACE_SIZE);

    let mut mappings = CubeMappings::new();
    add_mapping(&mut mappings, (0, Edge::Left), (1, Edge::Right));
    add_mapping(&mut mappings, (0, Edge::Bottom), (2, Edge::Right));
    add_mapping(&mut mappings, (0, Edge::Right), (3, Edge::Right));
    add_mapping(&mut mappings, (0, Edge::Top), (5, Edge::Bottom));

    add_mapping(&mut mappings, (1, Edge::Top), (5, Edge::Left));
    add_mapping(&mut mappings, (1, Edge::Left), (4, Edge::Left));
    add_mapping(&mut mappings, (1, Edge::Bottom), (2, Edge::Top));

    add_mapping(&mut mappings, (2, Edge::Left), (4, Edge::Top));
    add_mapping(&mut mappings, (2, Edge::Bottom), (3, Edge::Top));

    add_mapping(&mut mappings, (3, Edge::Left), (4, Edge::Right));
    add_mapping(&mut mappings, (3, Edge::Bottom), (5, Edge::Right));

    add_mapping(&mut mappings, (4, Edge::Bottom), (5, Edge::Top));

    Cube {
        faces: [face0, face1, face2, face3, face4, face5],
        mappings
    }
}

#[allow(dead_code)]
fn create_cube_from_sample_input(grid: &Grid<TerrainType>) -> Cube {
    let face0 = grid.extract(FACE_SIZE*2, 0,           FACE_SIZE, FACE_SIZE);
    let face1 = grid.extract(0,           FACE_SIZE,   FACE_SIZE, FACE_SIZE);
    let face2 = grid.extract(FACE_SIZE,   FACE_SIZE,   FACE_SIZE, FACE_SIZE);
    let face3 = grid.extract(FACE_SIZE*2, FACE_SIZE,   FACE_SIZE, FACE_SIZE);
    let face4 = grid.extract(FACE_SIZE*2, FACE_SIZE*2, FACE_SIZE, FACE_SIZE);
    let face5 = grid.extract(FACE_SIZE*3, FACE_SIZE*2, FACE_SIZE, FACE_SIZE);

    let mut mappings = CubeMappings::new();
    add_mapping(&mut mappings, (0, Edge::Left), (2, Edge::Top));
    add_mapping(&mut mappings, (0, Edge::Top), (1, Edge::Top));
    add_mapping(&mut mappings, (0, Edge::Right), (5, Edge::Right));
    add_mapping(&mut mappings, (0, Edge::Bottom), (3, Edge::Top));

    add_mapping(&mut mappings, (1, Edge::Left), (5, Edge::Bottom));
    add_mapping(&mut mappings, (1, Edge::Right), (2, Edge::Left));
    add_mapping(&mut mappings, (1, Edge::Bottom), (4, Edge::Bottom));

    add_mapping(&mut mappings, (2, Edge::Right), (3, Edge::Left));
    add_mapping(&mut mappings, (2, Edge::Bottom), (4, Edge::Left));

    add_mapping(&mut mappings, (3, Edge::Right), (5, Edge::Top));
    add_mapping(&mut mappings, (3, Edge::Bottom), (4, Edge::Top));

    add_mapping(&mut mappings, (4, Edge::Right), (5, Edge::Left));

    Cube {
        faces: [face0, face1, face2, face3, face4, face5],
        mappings
    }
}

fn flatten_cube_pos(p: CubePosition) -> Position {
    let (x, y) = match p.face {
        0 => (FACE_SIZE*2, 0),
        1 => (FACE_SIZE,   0),
        2 => (FACE_SIZE,   FACE_SIZE),
        3 => (FACE_SIZE,   FACE_SIZE*2),
        4 => (0,           FACE_SIZE*2),
        5 => (0,           FACE_SIZE*3),
        _ => panic!("bad face"),
    };

    Position{x:(x as i64),y:(y as i64)} + p.pos
}

#[derive(Debug)]
struct Cube {
    faces: [Grid<TerrainType>; 6],
    mappings: CubeMappings,
}

impl Cube {
    fn get_pos(&self, pos: CubePosition) -> &TerrainType {
        self.faces[pos.face as usize].get_pos(pos.pos)
    }
}

fn add_mapping(m: &mut CubeMappings, a: (i64, Edge), b: (i64, Edge)) {
    m.insert(a, b);
    m.insert(b, a);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Edge {
    Bottom,
    Top,
    Left,
    Right,
}

type CubeMappings = HashMap<(i64, Edge), (i64, Edge)>;

#[derive(Debug, Clone, Copy)]
struct CubePosition {
    face: i64,
    pos: Position,
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
    pos: CubePosition,
    dir: Direction,
}

fn get_password(s: &State) -> i64 {
    let flat_pos = flatten_cube_pos(s.pos);
    let row = flat_pos.y + 1;
    let col = flat_pos.x + 1;
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

fn get_next_cube_pos(pos: CubePosition, dir: Direction) -> CubePosition {
    CubePosition { face: pos.face, pos: get_next_pos(pos.pos, dir) }
}

fn transition_over_edge(pos: Position, from_edge: Edge, to_edge: Edge) -> Position {
    match (from_edge, to_edge) {
        (Edge::Bottom, Edge::Top) => Position{x: pos.x, y: pos.y-(FACE_SIZE as i64)},
        (Edge::Top, Edge::Bottom) => Position{x: pos.x, y: pos.y+(FACE_SIZE as i64)},
        (Edge::Left, Edge::Right) => Position{x: pos.x+(FACE_SIZE as i64), y: pos.y},
        (Edge::Right, Edge::Left) => Position{x: pos.x-(FACE_SIZE as i64), y: pos.y},

        (Edge::Left, Edge::Left) => Position{x: pos.x+1, y: (FACE_SIZE as i64)-pos.y-1},
        (Edge::Right, Edge::Right) => Position{x: pos.x-1, y: (FACE_SIZE as i64)-pos.y-1},
        (Edge::Bottom, Edge::Bottom) => Position{x: (FACE_SIZE as i64)-pos.x-1, y: pos.y-1},
        (Edge::Top, Edge::Top) => Position{x: (FACE_SIZE as i64)-pos.x-1, y: pos.y+1},

        (Edge::Bottom, Edge::Right) => Position { x: pos.y-1, y: pos.x },
        (Edge::Right, Edge::Bottom) => Position { x: pos.y, y: pos.x-1 },
        (Edge::Top, Edge::Left) => Position { x: pos.y+1, y: pos.x },
        (Edge::Left, Edge::Top) => Position { x: pos.y, y: pos.x+1 },

        (Edge::Bottom, Edge::Left) => Position { x: pos.y-(FACE_SIZE as i64), y: (FACE_SIZE as i64)-pos.x-1},
        (Edge::Left, Edge::Bottom) => Position {x: (FACE_SIZE as i64)-pos.y-1, y: pos.x+(FACE_SIZE as i64)},
        (Edge::Top, Edge::Right) => Position{x:pos.y+(FACE_SIZE as i64), y: (FACE_SIZE as i64)-pos.x-1},
        (Edge::Right, Edge::Top) => Position{x: (FACE_SIZE as i64)-pos.y-1, y:pos.x-(FACE_SIZE as i64)},
    }
}

fn get_dir_from_edge(e: Edge) -> Direction {
    match e {
        Edge::Bottom => Direction::Up,
        Edge::Left => Direction::Right,
        Edge::Top => Direction::Down,
        Edge::Right => Direction::Left,
    }
}

fn wrap_cube_pos(m: &CubeMappings, pos: CubePosition) -> (CubePosition, Option<Direction>) {
    let from_edge =
        if pos.pos.y == -1 { Edge::Top }
        else if pos.pos.y == (FACE_SIZE as i64) { Edge::Bottom }
        else if pos.pos.x == -1 { Edge::Left }
        else if pos.pos.x == (FACE_SIZE as i64) { Edge::Right }
        else { return (pos, None); };

    let &(new_face, new_edge) = &m[&(pos.face, from_edge)];
    let new_pos = transition_over_edge(pos.pos, from_edge, new_edge);
    (CubePosition { face: new_face, pos: new_pos }, Some(get_dir_from_edge(new_edge)))
}

fn get_next_cube_pos_wrapped(m: &CubeMappings, pos: CubePosition, dir: Direction) -> (CubePosition, Direction) {
    let new_pos_raw = get_next_cube_pos(pos, dir);
    let (new_pos_wrapped, new_dir) = wrap_cube_pos(m, new_pos_raw);
    (new_pos_wrapped, new_dir.unwrap_or(dir))
}

fn go_forward_one_step(cube: &Cube, state: &State) -> Option<State> {
    let (new_pos, new_dir) = get_next_cube_pos_wrapped(&cube.mappings, state.pos, state.dir);
    match cube.get_pos(new_pos) {
        TerrainType::Floor => { return Some(State{pos:new_pos,dir:new_dir}); }
        TerrainType::Wall => { return None; },
        TerrainType::OutOfBounds => { panic!("should never happen, pos: {:?}", &new_pos) },
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

fn go_forward(cube: &Cube, state: &State, steps: i32) -> State {
    let mut s = *state;
    for _ in 0..steps {
        match go_forward_one_step(cube, &s) {
            Some(new_s) => { s = new_s; }
            None => { return s; }
        }
    }

    s
}

fn apply_instruction(cube: &Cube, state: &State, instruction: Instruction) -> State {
    match instruction {
        Instruction::Forward(steps) => go_forward(cube, state, steps),
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum TerrainType {
    #[default]
    OutOfBounds,
    Floor,
    Wall,
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
struct Grid<T: Clone + Default> {
    width: usize,
    vec: Vec<T>,
}

#[allow(dead_code)]
impl<T: Copy + Default> Grid<T> {
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

    fn extract(&self, x: usize, y: usize, width: usize, height: usize) -> Self {
        let mut g = Self::new(width, height, T::default());
        for dy in 0..height {
            for dx in 0..width {
                g.set(dx, dy, *self.get(x + dx, y + dy));
            }
        }
        g
    }
}