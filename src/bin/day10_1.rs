use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("data/day10/input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let mut instructions_iter = lines.into_iter().map(|x| decode_instruction(&x.unwrap()));

    let mut state = State { register_x: 1, current_instruction: None, cycle_number: 1 };

    let mut sum = 0;

    let mut ended = false;
    while ended == false {
        if state.cycle_number == 20 
        || state.cycle_number == 60
        || state.cycle_number == 100
        || state.cycle_number == 140 
        || state.cycle_number == 180
        || state.cycle_number == 220 {
            let signal_strength = get_signal_strength(&state);
            sum += signal_strength;
            println!("{}: {}", state.cycle_number, signal_strength);
        }
        ended = tick_state(&mut state, &mut instructions_iter);
    }

    println!("sum: {}", sum);
}

#[derive(Debug)]
struct State {
    register_x: i32,
    current_instruction: Option<Instruction>,
    cycle_number: i32,
}

fn tick_state<T: Iterator<Item=Instruction>>(state: &mut State, iter: &mut T) -> bool {
    match state.current_instruction {
        None => {
            let next_instruction = iter.next();
            match next_instruction {
                None => return true,
                Some(Instruction::NoOp) => {
                    // do nothing
                },
                Some(Instruction::Add(num)) => {
                    state.current_instruction = Some(Instruction::Add(num));
                },
            }
        }
        Some(Instruction::NoOp) => {
            panic!("shouldn't happen, noop finishes on the cycle it starts");
        },
        Some(Instruction::Add(num)) => {
            // add always finishes this cycle
            state.register_x += num;
            state.current_instruction = None;

        },
    };
    state.cycle_number += 1;

    false
}

fn get_signal_strength(state: &State) -> i32 {
    state.register_x * state.cycle_number
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Instruction {
    NoOp,
    Add(i32),
}

fn decode_instruction(line: &str) -> Instruction {
    let parts = line.split(' ').collect::<Vec<_>>();
    match parts[0] {
        "addx" => Instruction::Add(parts[1].parse().unwrap()),
        "noop" => Instruction::NoOp,
        _ => panic!("invalid instruction: {}", parts[0]),
    }
}
