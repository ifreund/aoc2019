use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Bug,
    Empty,
}

#[aoc_generator(day24)]
pub fn parse_day24(input: &str) -> [[Tile; 5]; 5] {
    let mut bugs = [[Tile::Empty; 5]; 5];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            bugs[y][x] = match ch {
                '#' => Tile::Bug,
                '.' => Tile::Empty,
                _ => panic!("ERROR: {} is not a valid tile"),
            };
        }
    }
    bugs
}

fn step(bugs: &mut [[Tile; 5]; 5]) {
    let mut next_bugs = bugs.clone();
    for y in 0..5 {
        for x in 0..5 {
            let neighbor_count = {
                let mut count = 0;
                if x > 0 && bugs[y][x - 1] == Tile::Bug {
                    count += 1;
                }
                if y > 0 && bugs[y - 1][x] == Tile::Bug {
                    count += 1;
                }
                if x < 4 && bugs[y][x + 1] == Tile::Bug {
                    count += 1;
                }
                if y < 4 && bugs[y + 1][x] == Tile::Bug {
                    count += 1;
                }
                count
            };
            match (bugs[y][x], neighbor_count) {
                (Tile::Empty, 1) | (Tile::Empty, 2) => next_bugs[y][x] = Tile::Bug,
                (Tile::Bug, count) if count != 1 => next_bugs[y][x] = Tile::Empty,
                _ => (),
            }
        }
    }
    *bugs = next_bugs;
}

#[aoc(day24, part1)]
pub fn solve_day24_part1(input: &[[Tile; 5]; 5]) -> usize {
    let mut bugs = *input;
    let mut hist = HashSet::new();
    hist.insert(bugs);
    loop {
        step(&mut bugs);
        if !hist.insert(bugs) {
            // Break if we've seen this state before
            break;
        }
    }

    bugs.iter()
        .flatten()
        .enumerate()
        .filter_map(|(n, tile)| match tile {
            Tile::Bug => Some(1 << n),
            Tile::Empty => None,
        })
        .sum()
}

fn step_recursive(bugs: &mut HashMap<i32, [[Tile; 5]; 5]>) {
    let mut next_bugs = bugs.clone();
    for (level, grid) in bugs.iter() {
        for y in 0..5 {
            for x in 0..5 {
                if y == 2 && x == 2 {
                    // The center is recursively another grid
                    continue;
                }
                let neighbor_count = {
                    let mut count = 0;
                    // North
                    if y == 0 {
                        // check 2,1 on previous level
                        if let Some(prev_grid) = bugs.get(&(level - 1)) {
                            if prev_grid[1][2] == Tile::Bug {
                                count += 1;
                            }
                        }
                    } else if y == 3 && x == 2 {
                        // check bottom row of next level
                        if let Some(next_grid) = bugs.get(&(level + 1)) {
                            for tile in next_grid[4].iter() {
                                if *tile == Tile::Bug {
                                    count += 1;
                                }
                            }
                        }
                    } else {
                        if grid[y - 1][x] == Tile::Bug {
                            count += 1;
                        }
                    }
                    // South
                    if y == 4 {
                        // check 2,3 on previous level
                        if let Some(prev_grid) = bugs.get(&(level - 1)) {
                            if prev_grid[3][2] == Tile::Bug {
                                count += 1;
                            }
                        }
                    } else if y == 1 && x == 2 {
                        // check top row of next level
                        if let Some(next_grid) = bugs.get(&(level + 1)) {
                            for tile in next_grid[0].iter() {
                                if *tile == Tile::Bug {
                                    count += 1;
                                }
                            }
                        }
                    } else {
                        if grid[y + 1][x] == Tile::Bug {
                            count += 1;
                        }
                    }
                    // West
                    if x == 0 {
                        // check 1,2 on previous level
                        if let Some(prev_grid) = bugs.get(&(level - 1)) {
                            if prev_grid[2][1] == Tile::Bug {
                                count += 1;
                            }
                        }
                    } else if y == 2 && x == 3 {
                        // check right row of next level
                        if let Some(next_grid) = bugs.get(&(level + 1)) {
                            for tile in next_grid.iter() {
                                if tile[4] == Tile::Bug {
                                    count += 1;
                                }
                            }
                        }
                    } else {
                        if grid[y][x - 1] == Tile::Bug {
                            count += 1;
                        }
                    }
                    // East
                    if x == 4 {
                        // check 3,2 on previous level
                        if let Some(prev_grid) = bugs.get(&(level - 1)) {
                            if prev_grid[2][3] == Tile::Bug {
                                count += 1;
                            }
                        }
                    } else if y == 2 && x == 1 {
                        // check left row of next level
                        if let Some(next_grid) = bugs.get(&(level + 1)) {
                            for tile in next_grid.iter() {
                                if tile[0] == Tile::Bug {
                                    count += 1;
                                }
                            }
                        }
                    } else {
                        if grid[y][x + 1] == Tile::Bug {
                            count += 1;
                        }
                    }
                    count
                };
                match (grid[y][x], neighbor_count) {
                    (Tile::Empty, 1) | (Tile::Empty, 2) => {
                        next_bugs.get_mut(level).unwrap()[y][x] = Tile::Bug;
                        // Ensure that neighboring levels are tracked
                        next_bugs.entry(level - 1).or_insert([[Tile::Empty; 5]; 5]);
                        next_bugs.entry(level + 1).or_insert([[Tile::Empty; 5]; 5]);
                    }
                    (Tile::Bug, count) if count != 1 => {
                        next_bugs.get_mut(level).unwrap()[y][x] = Tile::Empty
                    }
                    _ => (),
                }
            }
        }
    }
    *bugs = next_bugs;
}

#[aoc(day24, part2)]
pub fn solve_day24_part2(input: &[[Tile; 5]; 5]) -> usize {
    let mut bugs = HashMap::new();
    bugs.insert(0, *input);
    let zero_grid = [[Tile::Empty; 5]; 5];
    bugs.insert(-1, zero_grid);
    bugs.insert(1, zero_grid);
    for _ in 0..200 {
        step_recursive(&mut bugs);
    }
    bugs.iter().fold(0, |acc, (_, grid)| {
        acc + grid
            .iter()
            .flatten()
            .filter(|tile| **tile == Tile::Bug)
            .count()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn step_test() {
        const INITIAL: &str = "....#\n\
                               #..#.\n\
                               #..##\n\
                               ..#..\n\
                               #....\n";
        const MIN_1: &str = "#..#.\n\
                             ####.\n\
                             ###.#\n\
                             ##.##\n\
                             .##..\n";
        let mut bugs = parse_day24(INITIAL);
        step(&mut bugs);
        assert_eq!(bugs, parse_day24(MIN_1));
    }
}
