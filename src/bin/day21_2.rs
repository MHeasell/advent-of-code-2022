use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

use lazy_static::lazy_static;
use regex::Regex;

const HUMAN_NAME: &str = "humn";

fn main() {
    let file = File::open("data/day21/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let exprs = lines.map(|x| parse_expr(&x.unwrap())).collect::<HashMap<_,_>>();

    let root = &exprs["root"];
    let solution = match root {
        Expr::Oper(left, _, right) => {
            if contains_var(&exprs, left, HUMAN_NAME) {
                solve_for_var(&exprs, left, eval_expr(&exprs, right), HUMAN_NAME)
            }
            else if contains_var(&exprs, right, HUMAN_NAME) {
                solve_for_var(&exprs, right, eval_expr(&exprs, left), HUMAN_NAME)
            }
            else {
                panic!("neither side contained {}", HUMAN_NAME)
            }
        }
        Expr::Const(_) => panic!("root contained constant")
    };

    dbg!(solution);
}

fn contains_var(hm: &HashMap<String, Expr>, root: &str, name: &str) -> bool {
    if root == name {
        return true
    }
    match &hm[root] {
        Expr::Const(_) => false,
        Expr::Oper(left, _, _) if left == name => true,
        Expr::Oper(_, _, right) if right == name => true,
        Expr::Oper(left, _, right) => contains_var(hm, left, name) || contains_var(hm, right, name)
    }
}

// really inefficient to have to keep looking inside the tree to figure out
// which half has the name we want, but it will do
fn solve_for_var(hm: &HashMap<String, Expr>, expr_name: &str, lhs: i64, var_name: &str) -> i64 {
    if expr_name == var_name {
        return lhs;
    }

    match &hm[expr_name] {
        Expr::Const(_) => panic!("var not found: {}", var_name),
        Expr::Oper(left, op, right) => {
            if contains_var(hm, left, var_name) {
                let right_val = eval_expr(hm, right);
                let new_lhs = find_l(lhs, *op, right_val);
                solve_for_var(hm, left, new_lhs, var_name)
            }
            else if contains_var(hm, right, var_name) {
                let left_val = eval_expr(hm, left);
                let new_lhs = find_r(lhs, left_val, *op);
                solve_for_var(hm, right, new_lhs, var_name)
            }
            else {
                panic!("neither side contained {}", var_name)
            }
        }
    }
}

fn eval_oper(a: i64, op: Op, b: i64) -> i64 {
    match op {
        Op::Plus => a + b,
        Op::Minus => a - b,
        Op::Mult => a * b,
        Op::Divide => a / b,
    }
}

// given a = l <op> r, find l
fn find_l(a: i64, op: Op, r: i64) -> i64 {
    match op {
        Op::Plus => a - r,
        Op::Minus => a + r,
        Op::Mult => a / r,
        Op::Divide => a * r,
    }
}

// given a = l <op> r, find r
fn find_r(a: i64, l: i64, op: Op) -> i64 {
    match op {
        Op::Plus => a - l,
        Op::Minus => l - a,
        Op::Mult => a / l,
        Op::Divide => l / a,
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

fn parse_op(s:&str) -> Op {
    match s {
        "+" => Op::Plus,
        "-" => Op::Minus,
        "*" => Op::Mult,
        "/" => Op::Divide,
        _ => panic!("bad op string")
    }
}

lazy_static! {
    static ref CONST_REGEX: Regex = Regex::new(r"^([a-z]+): (\d+)$").unwrap();
    static ref EXPR_REGEX: Regex = Regex::new(r"^([a-z]+): ([a-z]+) ([+*/-]) ([a-z]+)$").unwrap();
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
