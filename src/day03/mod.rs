use input::INPUT;
use itertools::Itertools;
use regex::{Match, Regex};

mod input;

pub fn run() {
    println!("--- Day 03 ---");

    solution_part_one();
    solution_part_two();

    println!();
}

fn solution_part_one() {
    let regex = Regex::new(r"mul\((?<num01>[0-9]+),(?<num02>[0-9]+)\)").unwrap();
    let res: i32 = regex.find_iter(INPUT).map(|m| calculate(&m, &regex)).sum();

    println!("Pt1 - Multiplication result: {res}");
}

fn solution_part_two() {
    // Replacing `don't` with `never` prevents issues when splitting with `do`.
    let input = INPUT.replace("don't", "never");
    let regex = Regex::new(r"mul\((?<num01>[0-9]+),(?<num02>[0-9]+)\)").unwrap();

    let split = split_inclusive_rhs(&input, "never")
        .iter()
        .flat_map(|s| s.split("do"))
        .filter(|s| !s.starts_with("never"))
        .join("");

    let res: i32 = regex.find_iter(&split).map(|m| calculate(&m, &regex)).sum();

    println!("Pt2 - Multiplication result: {res}");
}

fn calculate(m: &Match, regex: &Regex) -> i32 {
    let c = regex.captures(m.as_str()).unwrap();
    let num01: i32 = c["num01"].parse().unwrap();
    let num02: i32 = c["num02"].parse().unwrap();

    num01 * num02
}

/// Works similarly to [str::split_inclusive], but instead of including the delimiter at the end
/// of the LHS slice, it is placed at the start of the RHS slice.
fn split_inclusive_rhs(input: &str, delimiter: &str) -> Vec<String> {
    input
        .split(delimiter)
        .enumerate()
        .map(|(i, part)| {
            if i == 0 {
                part.to_owned()
            } else {
                format!("{delimiter}{part}")
            }
        })
        .collect()
}
