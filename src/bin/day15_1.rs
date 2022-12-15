use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day15/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    //let target_row_y = 10;
    let target_row_y = 2000000;

    let sensor_lines = lines.map(|x| parse_sensor_line(&x.unwrap())).collect::<Vec<_>>();

    let mut intervals = sensor_lines.iter().flat_map(|l| {
        let sensor_range = get_manhattan_distance(l.sensor_position, l.closest_beacon_position);
        get_row_range_covered(l.sensor_position, sensor_range, target_row_y)
    }).collect::<Vec<_>>();

    flatten_intervals(&mut intervals);

    let mut covered_cell_count = intervals.iter().map(|x| x.len()).sum::<i32>();

    // subtract beacons that live in the intervals
    let beacon_positions = sensor_lines.iter().map(|x| x.closest_beacon_position).collect::<HashSet<_>>();
    let beacons_in_range = beacon_positions.iter().filter(|p| p.y == target_row_y && intervals.iter().any(|interval| interval.contains_val(p.x))).count();
    covered_cell_count -= i32::try_from(beacons_in_range).unwrap();

    println!("{}", covered_cell_count);
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
    fn contains_val(&self, val: i32) -> bool {
        self.first <= val && self.last >= val
    }

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

    fn len(&self) -> i32 {
        self.last - self.first + 1
    }
}

fn flatten_intervals(acc: &mut Vec<Interval>) {
    acc.sort_by_key(|x| x.first);

    let mut i = 0;
    while i < acc.len()-1 {
        let a = acc[i];
        let b = acc[i+1];
        match a.combine_with(&b) {
            Some(c) => {
                acc.remove(i+1);
                acc[i] = c;
            }
            None => {
                i += 1;
            }
        }
    }
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
