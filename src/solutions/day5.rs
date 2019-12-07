use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn parse_day5(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_day5_part1(program: &[i32]) -> i32 {
    intcode::execute(program, &[1])
}

#[aoc(day5, part2)]
pub fn solve_day5_part2(program: &[i32]) -> i32 {
    intcode::execute(program, &[5])
}
