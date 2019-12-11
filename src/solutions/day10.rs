use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeMap;
use std::collections::HashSet;

#[aoc_generator(day10)]
pub fn parse_day10(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, loc)| {
                if loc == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

// Will be negative if on one side of the potential location and positive on the other
#[derive(PartialEq, Eq, Hash)]
struct Slope {
    rise: i32,
    run: i32,
}

impl Slope {
    fn new(rise: i32, run: i32) -> Self {
        if rise == 0 {
            Self {
                rise: 0,
                run: run.signum(),
            }
        } else if run == 0 {
            Self {
                rise: rise.signum(),
                run: 0,
            }
        } else {
            let gcd = gcd(rise.abs(), run.abs());
            Self {
                rise: rise / gcd,
                run: run / gcd,
            }
        }
    }
}

#[aoc(day10, part1)]
pub fn solve_day10_part1(input: &[(i32, i32)]) -> usize {
    input
        .iter()
        .map(|(x, y)| {
            let mut visible = HashSet::new();
            input
                .iter()
                .filter(|(roidx, roidy)| visible.insert(Slope::new(y - roidy, x - roidx)))
                .count()
        })
        .max()
        .unwrap()
        - 1
}

// Solution coordinates from part 1
const STATION_X: i32 = 23;
const STATION_Y: i32 = 29;

#[aoc(day10, part2)]
pub fn solve_day10_part2(input: &[(i32, i32)]) -> i32 {
    let roids = input
        .iter()
        .filter(|roid| **roid != (STATION_X, STATION_Y))
        .map(|(roidx, roidy)| {
            (
                ((((STATION_Y - roidy) as f64).atan2((STATION_X - roidx) as f64)
                    - std::f64::consts::FRAC_PI_2)
                    .rem_euclid(std::f64::consts::PI * 2.0)
                    * 1_000_000_000_000.0) as i64,
                (roidx, roidy),
            )
        })
        .collect::<Vec<_>>();

    let mut roidmap = BTreeMap::new();
    for (angle, roid) in roids {
        roidmap
            .entry(angle)
            .and_modify(|roidlist: &mut Vec<_>| roidlist.push(roid))
            .or_insert_with(|| vec![roid]);
    }

    let mut count = 1;
    loop {
        for (_, roidlist) in roidmap.iter_mut() {
            if let Some((idx, roid)) =
                roidlist
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, (roidx, roidy))| {
                        (**roidx - STATION_X).pow(2) + (**roidy - STATION_Y).pow(2)
                    })
            {
                if count == 200 {
                    return 100 * roid.0 + roid.1;
                }
                roidlist.remove(idx);
                count += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(6, 9), 3);
        assert_eq!(gcd(700092, 3249), 9);
    }

    #[test]
    fn day10_example1() {
        const INPUT: &str = "......#.#.\n\
                             #..#.#....\n\
                             ..#######.\n\
                             .#.#.###..\n\
                             .#..#.....\n\
                             ..#....#.#\n\
                             #..#....#.\n\
                             .##.#..###\n\
                             ##...#..#.\n\
                             .#....####";
        assert_eq!(solve_day10_part1(&parse_day10(INPUT)), 33);
    }

    #[test]
    fn day10_example2() {
        const INPUT: &str = "#.#...#.#.\n\
                             .###....#.\n\
                             .#....#...\n\
                             ##.#.#.#.#\n\
                             ....#.#.#.\n\
                             .##..###.#\n\
                             ..#...##..\n\
                             ..##....##\n\
                             ......#...\n\
                             .####.###.";
        assert_eq!(solve_day10_part1(&parse_day10(INPUT)), 35);
    }

    #[test]
    fn day10_example3() {
        const INPUT: &str = ".#..#..###\n\
                             ####.###.#\n\
                             ....###.#.\n\
                             ..###.##.#\n\
                             ##.##.#.#.\n\
                             ....###..#\n\
                             ..#.#..#.#\n\
                             #..#.#.###\n\
                             .##...##.#\n\
                             .....#.#..";
        assert_eq!(solve_day10_part1(&parse_day10(INPUT)), 41);
    }

    #[test]
    fn day10_example4() {
        const INPUT: &str = ".#..##.###...#######\n\
                             ##.############..##.\n\
                             .#.######.########.#\n\
                             .###.#######.####.#.\n\
                             #####.##.#.##.###.##\n\
                             ..#####..#.#########\n\
                             ####################\n\
                             #.####....###.#.#.##\n\
                             ##.#################\n\
                             #####.##.###..####..\n\
                             ..######..##.#######\n\
                             ####.##.####...##..#\n\
                             .#####..#.######.###\n\
                             ##...#.##########...\n\
                             #.##########.#######\n\
                             .####.#.###.###.#.##\n\
                             ....##.##.###..#####\n\
                             .#.#.###########.###\n\
                             #.#.#.#####.####.###\n\
                             ###.##.####.##.#..##";
        assert_eq!(solve_day10_part1(&parse_day10(INPUT)), 210);
    }
}
