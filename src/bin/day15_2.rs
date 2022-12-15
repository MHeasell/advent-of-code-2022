use std::{fs::File, io::{BufReader, BufRead}};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day15/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let sensor_lines = lines.map(|x| parse_sensor_line(&x.unwrap())).collect::<Vec<_>>();

    //let search_space_max = 20;
    let search_space_max = 4000000;

    for y in 0..=search_space_max {
        let mut intervals = sensor_lines.iter().flat_map(|l| {
            let sensor_range = get_manhattan_distance(l.sensor_position, l.closest_beacon_position);
            get_row_range_covered(l.sensor_position, sensor_range, y)
        }).collect::<Vec<_>>();

        flatten_intervals(&mut intervals);

        if let Some(x) = find_hole_in_intervals(0, search_space_max, &intervals) {
            println!("found it: x: {}, y: {}", x, y);
            let freq = (i64::from(x) * 4000000) + i64::from(y);
            println!("freq: {}", freq);
            return;
        }
    }
    panic!("not found!");
}

lazy_static! {
    static ref SENSOR_LINE_REGEX: Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
}
fn parse_sensor_line(line: &str) -> SensorLine {
    let captures = SENSOR_LINE_REGEX.captures(line).unwrap();

    SensorLine {
        sensor_position: Position{
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
        },
        closest_beacon_position: Position {
            x: captures[3].parse().unwrap(),
            y: captures[4].parse().unwrap()
        },
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Interval {
    first: i32,
    last: i32,
}

impl Interval {
    // fn contains_val(&self, val: i32) -> bool {
    //     self.first <= val && self.last >= val
    // }

    fn overlaps(&self, other: &Interval) -> bool {
        self.last >= other.first && self.first <= other.last
    }

    fn combine_with(&self, other: &Interval) -> Option<Interval> {
        if !self.overlaps(other) { return None; }
        Some(Interval {
            first: self.first.min(other.first),
            last: self.last.max(other.last),
        })
    }

    // fn len(&self) -> i32 {
    //     self.last - self.first + 1
    // }
}

fn find_hole_in_intervals(min: i32, max: i32, intervals: &[Interval]) -> Option<i32> {
    if intervals.is_empty() || intervals[0].first > min { return Some(min); }
    if intervals[intervals.len()-1].last < max { return Some(intervals[intervals.len()-1].last + 1); }

    intervals.windows(2).find_map(|slice| {
        let a = slice[0];
        let b = slice[1];
        if a.last + 1 < b.first { Some(a.last + 1) } else { None }
    })
}

fn flatten_intervals(intervals: &mut Vec<Interval>) {
    intervals.sort_by_key(|x| x.first);

    let mut write_index = 0;
    let mut read_index = 1;
    while read_index < intervals.len() {
        let a = intervals[write_index];
        let b = intervals[read_index];
        match a.combine_with(&b) {
            Some(c) => {
                intervals[write_index] = c;
            }
            None => {
                write_index += 1;
                intervals[write_index] = b;
            }
        }
        read_index += 1;
    }
    intervals.truncate(write_index+1);
}

fn get_row_range_covered(sensor_pos: Position, sensor_range: u32, row_y: i32) -> Option<Interval> {
    let row_distance = row_y.abs_diff(sensor_pos.y);
    if row_distance > sensor_range { return None; }

    let remaining_range = i32::try_from(sensor_range - row_distance).unwrap();

    Some(Interval {
        first: sensor_pos.x - remaining_range,
        last: sensor_pos.x + remaining_range,
    })
}

fn get_manhattan_distance(a: Position, b: Position) -> u32 {
    let delta_x = b.x.abs_diff(a.x);
    let delta_y = b.y.abs_diff(a.y);
    delta_x + delta_y
}

#[derive(Debug)]
struct SensorLine {
    sensor_position: Position,
    closest_beacon_position: Position,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manhattan_distance() {
        assert_eq!(get_manhattan_distance(Position{x:1, y:1}, Position{x:1, y:1}), 0);

        assert_eq!(get_manhattan_distance(Position{x:1, y:1}, Position{x:1, y:2}), 1);
        assert_eq!(get_manhattan_distance(Position{x:1, y:1}, Position{x:2, y:1}), 1);

        assert_eq!(get_manhattan_distance(Position{x:1, y:1}, Position{x:1, y:0}), 1);
        assert_eq!(get_manhattan_distance(Position{x:1, y:1}, Position{x:0, y:1}), 1);

        assert_eq!(get_manhattan_distance(Position{x:-1, y:-1}, Position{x:-1, y:-1}), 0);

        assert_eq!(get_manhattan_distance(Position{x:-1, y:-1}, Position{x:-1, y:-4}), 3);
        assert_eq!(get_manhattan_distance(Position{x:-1, y:-1}, Position{x:-4, y:-1}), 3);

        assert_eq!(get_manhattan_distance(Position{x:0, y:0}, Position{x:-4, y:-1}), 5);
        assert_eq!(get_manhattan_distance(Position{x:2, y:1}, Position{x:-4, y:-1}), 8);
    }

    #[test]
    fn test_get_row_range_covered() {
        assert_eq!(get_row_range_covered(Position{x:0,y:0}, 5, 5), Some(Interval{first:0, last:0}));
        assert_eq!(get_row_range_covered(Position{x:3,y:0}, 5, 5), Some(Interval{first:3, last:3}));
    }
}
