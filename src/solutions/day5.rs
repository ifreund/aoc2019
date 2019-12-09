use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn parse_day5(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_day5_part1(program: &[i64]) -> i64 {
    *intcode::execute(program, &[1]).last().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_day5_part2(program: &[i64]) -> i64 {
    *intcode::execute(program, &[5]).last().unwrap()
}
