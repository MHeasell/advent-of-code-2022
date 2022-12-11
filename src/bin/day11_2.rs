fn main() {
    // Sample input monkeys
    /*
    let mut monkeys = vec![
        Monkey{
            items: vec![79, 98],
            operation: Operation::Multiply(Var::Const(19)),
            test_divisor: 23,
            true_target: 2,
            false_target: 3,
            inspection_count: 0,
        },
        Monkey{
            items: vec![54, 65, 75, 74],
            operation: Operation::Add(Var::Const(6)),
            test_divisor: 19,
            true_target: 2,
            false_target: 0,
            inspection_count: 0,
        },
        Monkey{
            items: vec![79, 60, 97],
            operation: Operation::Multiply(Var::Old),
            test_divisor: 13,
            true_target: 1,
            false_target: 3,
            inspection_count: 0,
        },
        Monkey{
            items: vec![74],
            operation: Operation::Add(Var::Const(3)),
            test_divisor: 17,
            true_target: 0,
            false_target: 1,
            inspection_count: 0,
        },
    ];
    */

    // Puzzle input monkeys
    let mut monkeys = vec![
        Monkey{
            items: vec![71, 56, 50, 73],
            operation: Operation::Multiply(Var::Const(11)),
            test_divisor: 13,
            true_target: 1,
            false_target: 7,
            inspection_count: 0,
        },
        Monkey{
            items: vec![70, 89, 82],
            operation: Operation::Add(Var::Const(1)),
            test_divisor: 7,
            true_target: 3,
            false_target: 6,
            inspection_count: 0,
        },
        Monkey{
            items: vec![52, 95],
            operation: Operation::Multiply(Var::Old),
            test_divisor: 3,
            true_target: 5,
            false_target: 4,
            inspection_count: 0,
        },
        Monkey{
            items: vec![94, 64, 69, 87, 70],
            operation: Operation::Add(Var::Const(2)),
            test_divisor: 19,
            true_target: 2,
            false_target: 6,
            inspection_count: 0,
        },
        Monkey{
            items: vec![98, 72, 98, 53, 97, 51],
            operation: Operation::Add(Var::Const(6)),
            test_divisor: 5,
            true_target: 0,
            false_target: 5,
            inspection_count: 0,
        },
        Monkey{
            items: vec![79],
            operation: Operation::Add(Var::Const(7)),
            test_divisor: 2,
            true_target: 7,
            false_target: 0,
            inspection_count: 0,
        },
        Monkey{
            items: vec![77, 55, 63, 93, 66, 90, 88, 71],
            operation: Operation::Multiply(Var::Const(7)),
            test_divisor: 11,
            true_target: 2,
            false_target: 4,
            inspection_count: 0,
        },
        Monkey{
            items: vec![54, 97, 87, 70, 59, 82, 59],
            operation: Operation::Add(Var::Const(8)),
            test_divisor: 17,
            true_target: 1,
            false_target: 3,
            inspection_count: 0,
        },
    ];

    let all_lcm = monkeys.iter().map(|m| m.test_divisor).reduce(lcm).unwrap();
    println!("lcm: {}", all_lcm);

    for _ in 0..10000 {
        do_round(&mut monkeys, all_lcm);
    }

    let mut scores = monkeys.iter().map(|m| m.inspection_count).collect::<Vec<_>>();
    scores.sort();

    let final_thing = scores[scores.len()-1] * scores[scores.len()-2];

    println!("{}", final_thing);
}

#[derive(Debug)]
enum Var {
    Old,
    Const(u64),
}

#[derive(Debug)]
enum Operation {
    Add(Var),
    Multiply(Var),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    true_target: usize,
    false_target: usize,
    inspection_count: u64,
}

fn inspect(op: &Operation, item: u64) -> u64 {
    match op {
        Operation::Add(Var::Const(x)) => item + x,
        Operation::Add(Var::Old) => item + item,
        Operation::Multiply(Var::Const(x)) => item * x,
        Operation::Multiply(Var::Old) => item * item,
    }
}

// Terrible name lol
fn split_at_idx_mut<T>(items: &mut Vec<T>, i: usize) -> (&mut [T], &mut T, &mut [T]) {
    let (before, after) = items.split_at_mut(i);
    let (curr, after2) = after.split_first_mut().unwrap();
    (before, curr, after2)
}

fn do_round(monkeys: &mut Vec<Monkey>, modulo_val: u64) {
    for i in 0..monkeys.len() {
        let (monkeys_before, monkey, monkeys_after) = split_at_idx_mut(monkeys, i);
        // We'll assume a monkey can't throw to itself
        // so we can just clear all its items at the end.
        for item in &monkey.items {
            let new_item = inspect(&monkey.operation, *item) % modulo_val;
            monkey.inspection_count += 1;
            let target = if new_item % monkey.test_divisor == 0 {
                monkey.true_target
            } else {
                monkey.false_target
            };
            if target < i {
                monkeys_before[target].items.push(new_item);
            } else if target > i {
                monkeys_after[target-i-1].items.push(new_item);
            }
            else {
                panic!("monkey throws to itself");
            }
        }
        monkey.items.clear()
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    let x = gcd(a, b);
    (a * b) / x
    
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    if a > b {
        gcd(b, a % b)
    }
    else {
        gcd(a, b % a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(27, 33), 3);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(2, 4), 4);
        assert_eq!(lcm(3, 5), 15);
    }
}