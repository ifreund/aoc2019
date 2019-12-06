use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day6)]
pub fn parse_day6(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .map(|x| {
            let mut x = x.split(')');
            let orbited = x.next().unwrap().to_owned();
            let orbiter = x.next().unwrap().to_owned();
            (orbiter, orbited)
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_day6_part1(orbits: &HashMap<String, String>) -> usize {
    orbits.iter().fold(0, |mut count, (orbiter, _)| {
        let mut current = orbiter;
        while let Some(orbited) = orbits.get(current) {
            current = orbited;
            count += 1;
        }
        count
    })
}

#[aoc(day6, part2)]
pub fn solve_day6_part2(orbits: &HashMap<String, String>) -> usize {
    let get_path_to_root = |start: &str| {
        let mut path = HashSet::new();
        let mut current = start;
        while let Some(orbited) = orbits.get(current) {
            path.insert(orbited);
            current = orbited;
        }
        path
    };
    let you_path = get_path_to_root("YOU");
    let san_path = get_path_to_root("SAN");
    you_path.symmetric_difference(&san_path).count()
}
