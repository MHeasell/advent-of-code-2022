use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

fn main() {
    let file = File::open("data/day8/input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let lines_iter = lines.into_iter().map(|x| x.unwrap());

    let grid = parse_grid(lines_iter);

    let mut seen = HashSet::<Position>::new();

    // from top
    (0..grid.width).flat_map(|x| {
        let grid_ref = &grid;
        scan_seq((0..grid.height()).map(move |y| (Position{x,y}, grid_ref.get(x,y))))
    }).for_each(|v| {
        seen.insert(v);
    });

    // from bottom
    (0..grid.width).flat_map(|x| {
        let grid_ref = &grid;
        scan_seq((0..grid.height()).rev().map(move |y| (Position{x,y}, grid_ref.get(x,y))))
    }).for_each(|v| {
        seen.insert(v);
    });

    // from left
    (0..grid.height()).flat_map(|y| {
        let grid_ref = &grid;
        scan_seq((0..grid.width).map(move |x| (Position{x,y}, grid_ref.get(x,y))))
    }).for_each(|v| {
        seen.insert(v);
    });

    // from right 
    (0..grid.height()).flat_map(|y| {
        let grid_ref = &grid;
        scan_seq((0..grid.width).rev().map(move |x| (Position{x,y}, grid_ref.get(x,y))))
    }).for_each(|v| {
        seen.insert(v);
    });

    println!("{}", seen.len());
}

fn scan_seq<T: Iterator<Item=(Position, i32)>>(iter: T) -> impl Iterator<Item=Position> {
    iter.scan(-1, |tallest, (pos, val)| {
        if val > *tallest {
            *tallest = val;
            Some(Some(pos))
        } else { Some(None) }
    }).flatten()
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    width: usize,
    vec: Vec<i32>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {

        Self { width: width, vec: vec![0; width*height] }
    }

    fn height(&self) -> usize {
        self.vec.len() / self.width
    }

    fn get(&self, x: usize, y: usize) -> i32 {
        self.vec[(y*self.width)+x]
    }

    fn set(&mut self, x: usize, y: usize, val: i32) {
        self.vec[(y*self.width)+x] = val;
    }
}

fn parse_grid<T: Iterator<Item=String>>(lines_iter: T) -> Grid {
    let lines = lines_iter.collect::<Vec<_>>();
    let mut g = Grid::new(lines[0].len(), lines.len());
    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            g.set(x, y, c.to_string().parse::<i32>().unwrap());
        }
    }
    g
}