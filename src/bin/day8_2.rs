use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("data/day8/input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let lines_iter = lines.into_iter().map(|x| x.unwrap());

    let grid = parse_grid(lines_iter);
    let w = grid.width;
    let h = grid.height();

    let max_score = (0..w)
        .flat_map(|x| (0..h).map(move |y| (x, y)))
        .map(|(x,y)| compute_scenic_score(&grid, Position{x,y}))
        .max()
        .unwrap();

    println!("{}", max_score);
}

fn compute_scenic_score(g: &Grid, pos: Position) -> usize {
    let val = g.get(pos.x, pos.y);

    let right_count = count_visible(val, ((pos.x+1)..g.width).map(|x| g.get(x, pos.y)));

    let left_count = count_visible(val, (0..pos.x).rev().map(|x| g.get(x, pos.y)));

    let down_count = count_visible(val, ((pos.y+1)..g.height()).map(|y| g.get(pos.x, y)));

    let up_count = count_visible(val, (0..pos.y).rev().map(|y| g.get(pos.x, y)));


    right_count * left_count * up_count * down_count
}

fn count_visible<T: Iterator<Item=i32>>(height: i32, iter: T) -> usize {
    iter.scan(false, |done, v| {
        if *done {
            return None;
        }
        if v >= height {
            *done = true;
        }
        Some(v)
    }).count()
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