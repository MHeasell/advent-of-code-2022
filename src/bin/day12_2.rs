use std::{fs::File, io::{BufReader, BufRead}, collections::{VecDeque, HashSet}};

fn main() {
    let file = File::open("data/day12/input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let grid = parse_grid(lines.into_iter().map(|x| x.unwrap()));

    let mut start_pos: Option<Position> = None;
    for y in 0..grid.height() {
        for x in 0..grid.width {
            if is_start(*grid.get(x, y)) {
                start_pos = Some(Position{x, y});
                break;
            }
        }
    }

    let start_pos = start_pos.unwrap();

    let cost = find_path_cost(&grid, start_pos).unwrap();

    println!("min cost: {}", cost);
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
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

    fn get(&self, x: usize, y: usize) -> &T {
        assert!(self.is_in_bounds(x, y));
        &self.vec[(y*self.width)+x]
    }

    fn get_pos(&self, pos: Position) -> &T {
        self.get(pos.x, pos.y)
    }

    fn try_get(&self, x: usize, y: usize) -> Option<&T> {
        if self.is_in_bounds(x, y) {
            Some(self.get(x, y))
        }
        else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        assert!(self.is_in_bounds(x, y));
        self.vec[(y*self.width)+x] = val;
    }
}

fn get_elevation(c: u8) -> i32 {
    match c {
        b'a'..=b'z' => (c as i32) - ('a' as i32),
        b'S' => get_elevation(b'a'),
        b'E' => get_elevation(b'z'),
        _ => panic!("invalid char: {}", (c as char)),
    }
}

fn is_start(c: u8) -> bool {
    c == b'E'
}

fn is_goal(c: u8) -> bool {
    c == b'S' || c == b'a'
}

fn parse_grid<T: Iterator<Item=String>>(lines_iter: T) -> Grid<u8> {
    let lines = lines_iter.collect::<Vec<_>>();
    let mut g = Grid::new(lines[0].len(), lines.len(), 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            g.set(x, y, c);
        }
    }
    g
}

#[derive(Copy, Clone, Debug)]
struct Entry {
    pos: Position,
    cost: i32,
}


fn get_successors(grid: &Grid<u8>, e: &Entry) -> Vec<Entry> {
    let mut v = Vec::new();
    if e.pos.x > 0 {
        v.push(Position{x: e.pos.x-1, y: e.pos.y});
    }
    v.push(Position{x: e.pos.x+1, y: e.pos.y});
    if e.pos.y > 0 {
        v.push(Position{x: e.pos.x, y: e.pos.y-1});
    }
    v.push(Position{x: e.pos.x, y: e.pos.y+1});

    v.into_iter().filter(|p| is_reachable(grid, e.pos, *p)).map(|p| Entry{pos: p, cost: e.cost+1}).collect()
}

fn is_reachable(grid: &Grid<u8>, from: Position, to: Position) -> bool {
    let from_val = grid.get(from.x, from.y);
    let to_val = grid.try_get(to.x, to.y);
    match to_val {
        None => false,
        Some(v) => get_elevation(*from_val) - 1 <= get_elevation(*v)
    }
}

fn find_path_cost(grid: &Grid<u8>, start: Position) -> Option<i32> {
    let mut open_list = VecDeque::<Entry>::new();
    let mut closed_set = HashSet::<Position>::new();

    open_list.push_back(Entry{pos: start, cost: 0});

    loop {
        match open_list.pop_front() {
            None => return None,
            Some(candidate) if is_goal(*grid.get_pos(candidate.pos)) => { return Some(candidate.cost); }
            Some(candidate) => {
                closed_set.insert(candidate.pos);
                let successors = get_successors(grid, &candidate);
                for successor in &successors {
                    if closed_set.contains(&successor.pos) { continue; }
                    let existing_elem = open_list.iter().enumerate().find(|(_, elem)| elem.pos == successor.pos);
                    match existing_elem {
                        Some((_, elem)) if elem.cost <= successor.cost => { continue; }
                        Some((i, _)) => { open_list.remove(i); }
                        None => {}
                    };

                    let idx = open_list.iter().enumerate().find_map(|(i, elem)| if elem.cost > successor.cost { Some(i) } else { None });
                    open_list.insert(idx.unwrap_or(open_list.len()), *successor);
                }
            }
        }
    }
}