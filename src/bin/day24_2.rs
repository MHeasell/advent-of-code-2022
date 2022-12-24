use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, VecDeque}};

fn main() {
    let file = File::open("data/day24/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());

    let (blizzard_directions, w, h) = parse_blizzards(lines);

    let blizzards = make_blizzards(&blizzard_directions, w, h);

    let start = (1,0);

    let state = State {
        pos: start,
        time: 0,
    };

    let goal = (w-2, h-1);

    let path_cost = find_shortest_path(&blizzards, &state, goal).unwrap();

    let state2 = State {
        pos: goal,
        time: path_cost,
    };
    let path_cost_2 = find_shortest_path(&blizzards, &state2, start).unwrap();

    let state3 = State {
        pos: start,
        time: path_cost_2,
    };
    let path_cost_3 = find_shortest_path(&blizzards, &state3, goal).unwrap();

    println!("cost 1: {}", path_cost);
    println!("cost 2: {}", path_cost_2);
    println!("cost 3 (answer): {}", path_cost_3);
}

fn make_blizzards(dirs: &[((i32,i32), Direction)], w: i32, h: i32) -> Blizzards {
    let mut b = Blizzards {
        up: HashSet::new(),
        down: HashSet::new(),
        left: HashSet::new(),
        right: HashSet::new(),
        width: w,
        height: h,
    };

    for (pos, dir) in dirs {
        match dir {
            Direction::Up => { b.up.insert(*pos); }
            Direction::Down => { b.down.insert(*pos); }
            Direction::Left => { b.left.insert(*pos); }
            Direction::Right => { b.right.insert(*pos); }
        }
    }

    b
}

fn parse_direction(c: char) -> Option<Direction> {
    match c {
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        '^' => Some(Direction::Up),
        'v' => Some(Direction::Down),
        '.'|'#' => None,
        _ => panic!("invalid direction: {}", c),
    }
}

fn parse_blizzards<T: Iterator<Item=String>>(lines: T) -> (Vec<((i32, i32), Direction)>, i32, i32) {
    let lines_vec = lines.collect::<Vec<_>>();
    let w = lines_vec[0].len();
    let h = lines_vec.len();

    let mut v = Vec::new();
    for (y, line) in lines_vec.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(d) = parse_direction(c) {
                v.push(((x as i32, y as i32), d));
            }
        }
    }
    (v, w.try_into().unwrap(), h.try_into().unwrap())
}

#[derive(Copy, Clone, Debug)]
struct Entry {
    state: State,
    cost: i32,
}

fn wrap(val: i32, min: i32, max: i32) -> i32 {
    let base = val - min;
    let range = max - min + 1;
    let new_base = ((base % range) + range) % range;
    min + new_base
}

fn rewind_pos(blizzards: &Blizzards, p: (i32, i32), dir: Direction, t: i32) -> (i32, i32) {
    match dir {
        Direction::Up => (p.0, wrap(p.1+t, 1, blizzards.height-2)),
        Direction::Down => (p.0, wrap(p.1-t, 1, blizzards.height-2)),
        Direction::Left => (wrap(p.0+t, 1, blizzards.width-2), p.1),
        Direction::Right => (wrap(p.0-t, 1, blizzards.width-2), p.1),
    }
}

fn collides(blizzards: &Blizzards, s: &State) -> bool {
    if s.pos == (1, 0) || s.pos == (blizzards.width-2, blizzards.height-1) {
        return false;
    }

    [
        (Direction::Up, &blizzards.up),
        (Direction::Down, &blizzards.down),
        (Direction::Left, &blizzards.left),
        (Direction::Right, &blizzards.right)
    ].into_iter().any(|(dir, m)| {
        let p = rewind_pos(blizzards, s.pos, dir, s.time);
        m.contains(&p)
    })
}

fn is_pos_in_bounds(blizzards: &Blizzards, pos: (i32,i32)) -> bool {
    if pos == (1, 0) || pos == (blizzards.width-2, blizzards.height-1) {
        return true;
    }

    pos.0 >= 1 && pos.0 <= blizzards.width-2 && pos.1 >= 1 && pos.1 <= blizzards.height - 2
}

fn get_successors(blizzards: &Blizzards, e: &Entry) -> Vec<Entry> {
    let new_states = [
        State{pos: (e.state.pos.0,e.state.pos.1), time: e.state.time+1},
        State{pos: (e.state.pos.0+1,e.state.pos.1), time: e.state.time+1},
        State{pos: (e.state.pos.0-1,e.state.pos.1), time: e.state.time+1},
        State{pos: (e.state.pos.0,e.state.pos.1+1), time: e.state.time+1},
        State{pos: (e.state.pos.0,e.state.pos.1-1), time: e.state.time+1},
    ];

    new_states.into_iter().filter(|s| is_pos_in_bounds(blizzards, s.pos) && !collides(blizzards, s))
    .map(|s| Entry{cost:s.time, state: s})
    .collect()
}

fn find_shortest_path(blizzards: &Blizzards, s: &State, goal: (i32, i32)) -> Option<i32> {

    let mut open_list = VecDeque::new();
    let mut closed_set = HashSet::new();

    open_list.push_back(Entry{cost: 0, state: *s});

    // let mut parents = HashMap::new();

    loop {
        match open_list.pop_front() {
            None => return None,
            Some(entry) if entry.state.pos == goal => {
                // let mut parent = parents.get(&entry.state);
                // while let Some(p) = parent {
                //     println!("parent: {:?}", &p);
                //     parent = parents.get(p);
                // }

                return Some(entry.cost);
            }
            Some(entry) => {
                // println!("exploring: {:?}", &entry);
                closed_set.insert(entry.state);
                let successors = get_successors(blizzards, &entry);
                for successor in &successors {
                    if closed_set.contains(&successor.state) { continue; }
                    let existing_elem = open_list.iter().enumerate().find(|(_, elem)| elem.state == successor.state);
                    match existing_elem {
                        Some((_, elem)) if elem.cost <= successor.cost => { continue; }
                        Some((i, _)) => { open_list.remove(i); }
                        None => {}
                    };

                    let idx = open_list.iter().enumerate().find_map(|(i, elem)| if elem.cost > successor.cost { Some(i) } else { None });
                    open_list.insert(idx.unwrap_or(open_list.len()), *successor);

                    // parents.insert(successor.state, entry.state);
                }
            }
        }
    }
}


#[derive(Debug)]
struct Blizzards {
    up: HashSet<(i32, i32)>,
    down: HashSet<(i32, i32)>,
    left: HashSet<(i32, i32)>,
    right: HashSet<(i32, i32)>,

    width: i32,
    height: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: (i32, i32),
    time: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collides() {
        let my_blizz = 
        "\
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#";

        let stuff = parse_blizzards(my_blizz.lines().map(|x| x.to_string()));
        let b = make_blizzards(&stuff.0, stuff.1, stuff.2);

        assert!(collides(&b, &State{ pos: (1, 2), time: 0 }));
        assert!(!collides(&b, &State{ pos: (2, 2), time: 0 }));
        assert!(!collides(&b, &State{ pos: (3, 2), time: 0 }));
        assert!(!collides(&b, &State{ pos: (4, 2), time: 0 }));
        assert!(!collides(&b, &State{ pos: (5, 2), time: 0 }));

        assert!(!collides(&b, &State{ pos: (1, 2), time: 1 }));
        assert!(collides(&b, &State{ pos: (2, 2), time: 1 }));
        assert!(!collides(&b, &State{ pos: (3, 2), time: 1 }));
        assert!(!collides(&b, &State{ pos: (4, 2), time: 1 }));
        assert!(!collides(&b, &State{ pos: (5, 2), time: 1 }));

        assert!(collides(&b, &State{ pos: (1, 2), time:  5 }));
        assert!(!collides(&b, &State{ pos: (2, 2), time: 5 }));
        assert!(!collides(&b, &State{ pos: (3, 2), time: 5 }));
        assert!(!collides(&b, &State{ pos: (4, 2), time: 5 }));
        assert!(!collides(&b, &State{ pos: (5, 2), time: 5 }));

        
    }

    #[test]
    fn test_collides_2() {

        let my_blizz = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";


        let stuff = parse_blizzards(my_blizz.lines().map(|x| x.to_string()));
        let b = make_blizzards(&stuff.0, stuff.1, stuff.2);

        assert!(!collides(&b, &State{ pos:(2,1),time:5}));

    }
}