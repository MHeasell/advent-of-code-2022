use std::{fs::File, io::{BufReader, BufRead}, iter::Peekable};

fn main() {
    let file = File::open("data/day13/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut sum = 0;

    let mut current_index = 1;

    loop {
        let first = parse_packet(&mut lines.next().unwrap().unwrap().chars().peekable());
        let second = parse_packet(&mut lines.next().unwrap().unwrap().chars().peekable());

        println!("first: {:?}", first);
        println!("second: {:?}", second);

        let in_order = are_lists_in_order(&first, &second);
        println!("are in order?: {:?}", in_order);
        if in_order == Some(true) {
            sum += current_index;
        }

        if lines.next().is_none() {
            break;
        }

        println!();
        current_index += 1;
    }

    println!("sum: {}", sum);
}

#[derive(Debug)]
enum PacketItem {
    Num(i32),
    List(Vec<PacketItem>),
}

fn are_nums_in_order(left: i32, right: i32) -> Option<bool> {
    if left < right { Some(true) }
    else if left > right { Some(false) }
    else { None }
}

fn are_lists_in_order(left: &[PacketItem], right: &[PacketItem]) -> Option<bool> {
    if left.is_empty() && right.is_empty() { None }
    else if left.is_empty() { Some(true) }
    else if right.is_empty() { Some(false) }
    else { 
        match are_in_order(&left[0], &right[0]) {
            Some(x) => Some(x),
            None => are_lists_in_order(&left[1..], &right[1..])
        }
    }
}

fn are_in_order(left: &PacketItem, right: &PacketItem) -> Option<bool> {
    match (left, right) {
        (PacketItem::Num(i), PacketItem::Num(j)) => are_nums_in_order(*i, *j),
        (PacketItem::List(v), PacketItem::List(u)) => are_lists_in_order(v, u),
        (PacketItem::List(v), PacketItem::Num(j)) => are_lists_in_order(v, &[PacketItem::Num(*j)]),
        (PacketItem::Num(i), PacketItem::List(u)) => are_lists_in_order(&[PacketItem::Num(*i)], u),
    }
}

fn parse_number<T: Iterator<Item=char>>(s: &mut Peekable<T>) -> i32 {
    let mut num_str = String::new();
    loop {
        match s.peek() {
            Some(x) if x.is_ascii_digit() => { num_str.push(s.next().unwrap()); }
            _ => { break; }
        }
    }
    num_str.parse().unwrap()
}


fn parse_item<T: Iterator<Item=char>>(s: &mut Peekable<T>) -> PacketItem {
    match s.peek() {
        Some('[') => PacketItem::List(parse_packet(s)),
        Some(_) => PacketItem::Num(parse_number(s)),
        None => panic!("unexpected end of input"),
    }
}

fn parse_packet<T: Iterator<Item=char>>(s: &mut Peekable<T>) -> Vec<PacketItem> {
    let mut v = Vec::new();
    assert!(s.next().unwrap() == '[');
    match s.peek() {
        Some(']') => {
            s.next();
            return v;
        }
        Some(_) => { v.push(parse_item(s)); }
        None => { panic!("unexpected end of input"); }
    }

    loop {
        match s.peek() {
            Some(']') => { 
                s.next();
                return v;
            }
            Some(',') => {
                s.next();
                v.push(parse_item(s));
            }
            Some(_) => { panic!("unexpected input"); }
            None => { panic!("unexpected end of input"); }
        }
    }
}
