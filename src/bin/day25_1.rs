use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("data/day25/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap());

    let sum = lines.map(|n| parse_snafu_number(&n)).sum::<i64>();

    println!("sum: {}", sum);
    println!("snafu: {}", format_snafu_number(sum));

}


fn parse_snafu_number(line: &str) -> i64 {
    let mut acc = 0;
    for c in line.chars() {
        acc *= 5;
        acc += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("invalid digit '{}'", c),
        }
    }
    acc
}

fn format_snafu_number(mut num: i64) -> String {
    let mut buf = String::new();

    // dbg!(num);

    loop {
        let ones_val = ((num+2) % 5)-2;
        // dbg!(ones_val);
        buf.insert(0, match ones_val {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("should never get here"),
        });
        num = (num-ones_val)/5;
        // dbg!(num);
        if num == 0 {
            return buf;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_snafu_number_specific() {
        assert_eq!("1=", format_snafu_number(3));
    }

    #[test]
    fn test_parse_snafu_number() {
        let test_cases = [
(        1,             "1"),
(        2,             "2"),
(        3,            "1="),
(        4,            "1-"),
(        5,            "10"),
(        6,            "11"),
(        7,            "12"),
(        8,            "2="),
(        9,            "2-"),
(       10,            "20"),
(       11,            "21"),
(       12,            "22"),
(       13,           "1=="),
(       14,           "1=-"),
(       15,           "1=0"),
(       20,           "1-0"),
(     2022,        "1=11-2"),
(    12345,       "1-0---0"),
(314159265, "1121-1110-1=0"),
        ];

        for (num, snafu) in test_cases {
            assert_eq!(num, parse_snafu_number(snafu), "snafu: {} -> num: {}", snafu, num);
            assert_eq!(snafu, format_snafu_number(num), "num: {} -> snafu {}", num, snafu);
        }
    }
}