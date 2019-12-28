use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day19)]
pub fn parse_day19(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day19, part1)]
pub fn solve_day19_part1(program: &[i64]) -> i64 {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            let res = *intcode::execute(program, &[x, y]).last().unwrap();
            count += res;
            print!("{}", if res == 1 { '#' } else { '.' });
        }
        println!();
    }
    count
}

fn check_coord(x: i64, y: i64, program: &[i64], cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(value) = cache.get(&(x, y)) {
        return *value;
    }
    let value = *intcode::execute(program, &[x, y]).last().unwrap();
    cache.insert((x, y), value);
    value
}

fn check(x: i64, y: i64, program: &[i64], cache: &mut HashMap<(i64, i64), i64>) -> bool {
    for y in y..(y + 100) {
        for x in x..(x + 100) {
            if 0 == check_coord(x, y, program, cache) {
                return false;
            }
        }
    }
    true
}

#[aoc(day19, part2)]
fn solve_day19_part2(program: &[i64]) -> i64 {
    let mut cache = HashMap::new();
    for y in 1000..10000 {
        for x in 0..4000 {
            if check(x, y, program, &mut cache) {
                return 10000 * x + y;
            }
        }
    }
    0
}
