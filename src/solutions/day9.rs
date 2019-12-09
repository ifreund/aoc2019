use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn parse_day9(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_day9_part1(program: &[i64]) -> i64 {
    *intcode::execute(program, &[1]).last().unwrap()
}

#[aoc(day9, part2)]
pub fn solve_day9_part2(program: &[i64]) -> i64 {
    *intcode::execute(program, &[2]).last().unwrap()
}
