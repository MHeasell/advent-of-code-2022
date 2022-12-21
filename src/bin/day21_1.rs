use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let file = File::open("data/day21/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    //let mut exprs = lines.map(|x| parse_expr(&x.unwrap())).collect::<Vec<_>>();

    let exprs = lines.map(|x| parse_expr(&x.unwrap())).collect::<HashMap<_,_>>();

    let result = eval_expr(&exprs, "root");

    dbg!(result);
}

lazy_static! {
    static ref CONST_REGEX: Regex = Regex::new(r"^([a-z]+): (\d+)$").unwrap();
    static ref EXPR_REGEX: Regex = Regex::new(r"^([a-z]+): ([a-z]+) ([+*/-]) ([a-z]+)$").unwrap();
}

fn eval_oper(a: i64, op: Op, b: i64) -> i64 {
    match op {
        Op::Plus => a + b,
        Op::Minus => a - b,
        Op::Mult => a * b,
        Op::Divide => a / b,
    }
}

fn eval_expr(hm: &HashMap<String, Expr>, name: &str) -> i64 {
    let expr = &hm[name];
    match expr {
        Expr::Const(i) => *i,
        Expr::Oper(a, op, b) => {
            let a_val = eval_expr(hm, &a);
            let b_val = eval_expr(hm, &b);
            eval_oper(a_val, *op, b_val)
        }
    }
}

fn parse_op(s:&str) -> Op {
    match s {
        "+" => Op::Plus,
        "-" => Op::Minus,
        "*" => Op::Mult,
        "/" => Op::Divide,
        _ => panic!("bad op string")
    }
}

fn parse_expr(line: &str) -> (String, Expr) {
    if let Some(captures) = CONST_REGEX.captures(line) {
        return (captures[1].to_string(), Expr::Const(captures[2].parse().unwrap()));
    }

    if let Some(captures) = EXPR_REGEX.captures(line) {
        return (captures[1].to_string(), Expr::Oper(captures[2].to_string(), parse_op(&captures[3]), captures[4].to_string()));
    }

    panic!("unmatched line!")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Plus,
    Minus,
    Mult,
    Divide,
}

#[derive(Debug, Clone)]
enum Expr {
    Const(i64),
    Oper(String, Op, String),
}
