use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque};

fn main() {
    let file = File::open("data/day20/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut numbers = lines.map(|x| x.unwrap().parse::<i32>().unwrap()).enumerate().collect::<VecDeque<_>>();

    mix(&mut numbers);

    let zero_idx = numbers.iter().position(|x| x.1 == 0).unwrap();
    let c1_idx = (zero_idx + 1000) % numbers.len();
    let c2_idx = (zero_idx + 2000) % numbers.len();
    let c3_idx = (zero_idx + 3000) % numbers.len();

    let sum = [c1_idx, c2_idx, c3_idx].into_iter().map(|i| numbers[i].1).sum::<i32>();

    println!("sum: {}", sum);
}

fn mix(numbers: &mut VecDeque<(usize, i32)>) {

    let mut i = 0;

    let len = i32::try_from(numbers.len()).unwrap();

    for original_index in 0..numbers.len() {
        loop {
            let v = numbers[i];
            if v.0 != original_index {
                i = (i + 1) % numbers.len();
                continue;
            }
            move_thing(numbers, i, v.1);

            let idx_offset = v.1 / len + (if v.1 == 0 { 0 } else if v.1 < 0 { -1 } else { 1 });

            i = (i + 1) % numbers.len();
            i = mod_sub_2(i, idx_offset, numbers.len());
            break;
        }
    }
}

fn move_thing<T>(numbers: &mut VecDeque<T>, idx: usize, places: i32) {
    let len = numbers.len();
    if places > 0 {
        let places_usize = usize::try_from(places).unwrap();
        for i in 0..places_usize {
            let idx_a = (idx+i) % len;
            let idx_b = (idx+i+1) % len;
            numbers.swap(idx_a, idx_b);
        }
    }
    else {
        let places_usize = usize::try_from(places.abs()).unwrap();
        for i in 0..places_usize {
            let idx_a = mod_sub(idx, i, len);
            let idx_b = mod_sub(idx, i+1, len);
            numbers.swap(idx_a, idx_b);
        }
    }
}

fn mod_sub(mut a: usize, b: usize, len: usize) -> usize {
    while a < b {
        a += len;
    }
    (a - b) % len
}

fn mod_sub_2(a: usize, b: i32, len: usize) -> usize {
    if b < 0 {
        return (a + usize::try_from(b.abs()).unwrap()) % len;
    }

    mod_sub(a, usize::try_from(b).unwrap(), len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let mut v = VecDeque::from_iter([1,2,-3,3,-2,0,4].into_iter().enumerate());
        mix(&mut v);
        // assert_eq!(v.into_iter().map(|x| x.1).collect::<VecDeque<_>>(), VecDeque::from([1,2,-3,4,0,3,-2]));
        assert_eq!(v.into_iter().map(|x| x.1).collect::<VecDeque<_>>(), VecDeque::from([-2,1,2,-3,4,0,3]));
    }

    #[test]
    fn test_move_thing_forward() {
        let mut v = VecDeque::from([1,2,3,4,5]);
        move_thing(&mut v, 3, 1);
        assert_eq!(v, VecDeque::from([1,2,3,5,4]));
    }

    #[test]
    fn test_move_thing_zero() {
        let mut v = VecDeque::from([1,2,3,4,5]);
        move_thing(&mut v, 2, 0);
        assert_eq!(v, VecDeque::from([1,2,3,4,5]));
    }

    #[test]
    fn test_move_thing_backward() {
        let mut v = VecDeque::from([1,2,3,4,5]);
        move_thing(&mut v, 3, -1);
        assert_eq!(v, VecDeque::from([1,2,4,3,5]));
    }

    #[test]
    fn test_move_thing_forward_wrap() {
        let mut v = VecDeque::from([1,2,3,4,5]);
        move_thing(&mut v, 4, 1);
        assert_eq!(v, VecDeque::from([5,2,3,4,1]));
    }

    #[test]
    fn test_move_thing_backward_wrap() {
        let mut v = VecDeque::from([1,2,3,4,5]);
        move_thing(&mut v, 0, -1);
        assert_eq!(v, VecDeque::from([5,2,3,4,1]));
    }
}
