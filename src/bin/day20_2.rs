use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque};

fn main() {
    let file = File::open("data/day20/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut numbers = lines.map(|x| x.unwrap().parse::<i64>().unwrap()).enumerate().collect::<VecDeque<_>>();

    let decryption_key = 811589153;

    numbers.iter_mut().for_each(|x| x.1 *= decryption_key);

    // Index tracking original position -> position the number is at now
    let mut numbers_index = numbers.iter().map(|x| x.0).collect::<Vec<_>>();

    for _ in 0..10 {
        mix(&mut numbers, &mut numbers_index);
    }

    let zero_idx = numbers.iter().position(|x| x.1 == 0).unwrap();
    let c1_idx = (zero_idx + 1000) % numbers.len();
    let c2_idx = (zero_idx + 2000) % numbers.len();
    let c3_idx = (zero_idx + 3000) % numbers.len();

    let sum = [c1_idx, c2_idx, c3_idx].into_iter().map(|i| numbers[i].1).sum::<i64>();

    println!("sum: {}", sum);
}

fn mix(numbers: &mut VecDeque<(usize, i64)>, numbers_index: &mut [usize]) {
    for numbers_index_pos in 0..numbers_index.len() {
        let idx_of_num_to_move = numbers_index[numbers_index_pos];

        let v = numbers[idx_of_num_to_move];
        move_thing(numbers, idx_of_num_to_move, v.1, numbers_index);
    }
}

fn move_thing(numbers: &mut VecDeque<(usize, i64)>, idx: usize, mut places: i64, numbers_index: &mut [usize]) {
    let len = numbers.len();

    // Moving by (len-1) places is equivalent to a list rotation.
    // However since ultimately the rotation of the list doesn't matter to our caller
    // we'll avoid unnecessary work and just not do anything at all in this case.
    places = wrap_to_range(places, (len as i64) - 1);

    if places > 0 {
        let places_usize = usize::try_from(places).unwrap();
        for i in 0..places_usize {
            let idx_a = (idx+i) % len;
            let idx_b = (idx+i+1) % len;
            fancy_swap(numbers, idx_a, idx_b, numbers_index);
        }
    }
    else {
        let places_usize = usize::try_from(places.abs()).unwrap();
        for i in 0..places_usize {
            let idx_a = mod_sub(idx, i, len);
            let idx_b = mod_sub(idx, i+1, len);
            fancy_swap(numbers, idx_a, idx_b, numbers_index);
        }
    }
}

/// Wraps val so that -range < val < range
/// via repeated addition / subtraction of range
fn wrap_to_range(val: i64, range: i64) -> i64 {
    val % range
}

fn fancy_swap(numbers: &mut VecDeque<(usize, i64)>, idx_a: usize, idx_b: usize, numbers_index: &mut [usize]) {
    let v_a = numbers[idx_a];
    let v_b = numbers[idx_b];
    numbers.swap(idx_a, idx_b);

    numbers_index[v_a.0] = idx_b;
    numbers_index[v_b.0] = idx_a;
}

fn mod_sub(mut a: usize, b: usize, len: usize) -> usize {
    while a < b {
        a += len;
    }
    (a - b) % len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let mut v = VecDeque::from_iter([1,2,-3,3,-2,0,4].into_iter().enumerate());
        let mut index = vec![0,1,2,3,4,5,6];
        mix(&mut v, &mut index);
        assert_eq!(v.into_iter().map(|x| x.1).collect::<VecDeque<_>>(), VecDeque::from([-2,1,2,-3,4,0,3]));
        assert_eq!(index, vec![1, 2, 3, 6, 0, 5, 4]);
    }

    #[test]
    fn test_move_thing_forward() {
        let mut v = VecDeque::from_iter([1,2,3,4,5].into_iter().enumerate());
        let mut index = vec![0,1,2,3,4];
        move_thing(&mut v, 3, 1, &mut index);
        assert_eq!(v, VecDeque::from([(0,1),(1,2),(2,3),(4,5),(3,4)]));
        assert_eq!(index, vec![0,1,2,4,3]);
    }

    #[test]
    fn test_move_thing_zero() {
        let mut v = VecDeque::from_iter([1,2,3,4,5].into_iter().enumerate());
        let mut index = vec![0,1,2,3,4];
        move_thing(&mut v, 2, 0, &mut index);
        assert_eq!(v, VecDeque::from([(0,1),(1,2),(2,3),(3,4),(4,5)]));
        assert_eq!(index, vec![0,1,2,3,4]);
    }

    #[test]
    fn test_move_thing_backward() {
        let mut v = VecDeque::from_iter([1,2,3,4,5].into_iter().enumerate());
        let mut index = vec![0,1,2,3,4];
        move_thing(&mut v, 3, -1, &mut index);
        assert_eq!(v, VecDeque::from([(0,1),(1,2),(3,4),(2,3),(4,5)]));
        assert_eq!(index, vec![0,1,3,2,4]);
    }

    #[test]
    fn test_move_thing_forward_wrap() {
        let mut v = VecDeque::from_iter([1,2,3,4,5].into_iter().enumerate());
        let mut index = vec![0,1,2,3,4];
        move_thing(&mut v, 4, 1, &mut index);
        assert_eq!(v, VecDeque::from([(4,5),(1,2),(2,3),(3,4),(0,1)]));
        assert_eq!(index, vec![4,1,2,3,0]);
    }

    #[test]
    fn test_move_thing_backward_wrap() {
        let mut v = VecDeque::from_iter([1,2,3,4,5].into_iter().enumerate());
        let mut index = vec![0,1,2,3,4];
        move_thing(&mut v, 0, -1, &mut index);
        assert_eq!(v, VecDeque::from([(4,5),(1,2),(2,3),(3,4),(0,1)]));
        assert_eq!(index, vec![4,1,2,3,0]);
    }

    #[test]
    fn test_move_thing_forward_overwrap() {
        let mut v = VecDeque::from_iter([1,2,3,4,5].into_iter().enumerate());
        let mut index = vec![0,1,2,3,4];
        move_thing(&mut v, 1, 5, &mut index);
        assert_eq!(v, VecDeque::from([(0,1),(2,3),(1,2),(3,4),(4,5)]));
        assert_eq!(index, vec![0,2,1,3,4]);
    }

    #[test]
    fn test_move_thing_backward_overwrap() {
        let mut v = VecDeque::from_iter([1,2,3,4,5].into_iter().enumerate());
        let mut index = vec![0,1,2,3,4];
        move_thing(&mut v, 3, -5, &mut index);
        assert_eq!(v, VecDeque::from([(0,1),(1,2),(3,4),(2,3),(4,5)]));
        assert_eq!(index, vec![0,1,3,2,4]);
    }
}
