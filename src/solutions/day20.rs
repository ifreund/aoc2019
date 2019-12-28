use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

#[aoc_generator(day20)]
pub fn parse_day20(
    input: &str,
) -> (
    HashSet<Point>,
    HashMap<Point, Point>,
    HashMap<Point, Point>,
    Point,
    Point,
) {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut map = HashSet::new();
    let mut outer_portals = HashMap::new();
    let mut inner_portals = HashMap::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            match input[y][x] {
                '.' => {
                    map.insert((x, y));
                }
                '#' | ' ' => (),
                ch1 => {
                    let is_ch = |ch| ch != '.' && ch != '#' && ch != ' ';
                    if let Some((ch2, portal)) = if x + 1 < input[y].len() && is_ch(input[y][x + 1])
                    {
                        let portal = if let Some('.') = input[y].get(x + 2) {
                            (x + 2, y)
                        } else if let Some('.') = input[y].get(x - 1) {
                            (x - 1, y)
                        } else {
                            panic!("ERROR: failed to read portal at {},{}", x, y);
                        };
                        Some((input[y][x + 1], portal))
                    } else if y + 1 < input.len() && is_ch(input[y + 1][x]) {
                        let portal = if y + 2 < input.len() && input[y + 2][x] == '.' {
                            (x, y + 2)
                        } else if y - 1 > 0 && input[y - 1][x] == '.' {
                            (x, y - 1)
                        } else {
                            panic!("ERROR: failed to read portal at {},{}", x, y);
                        };
                        Some((input[y + 1][x], portal))
                    } else {
                        None
                    } {
                        // Outer portal
                        if portal.0 == 2
                            || portal.1 == 2
                            || portal.0 == input[portal.1].len() - 3
                            || portal.1 == input.len() - 3
                        {
                            if let Some((first, _)) = outer_portals.get_mut(&(ch1, ch2)) {
                                *first = portal;
                            } else {
                                outer_portals.insert((ch1, ch2), (portal, (0, 0)));
                            }
                            if let Some((_, second)) = inner_portals.get_mut(&(ch1, ch2)) {
                                *second = portal;
                            } else {
                                inner_portals.insert((ch1, ch2), ((0, 0), portal));
                            }
                        } else {
                            // Inner portal
                            if let Some((first, _)) = inner_portals.get_mut(&(ch1, ch2)) {
                                *first = portal;
                            } else {
                                inner_portals.insert((ch1, ch2), (portal, (0, 0)));
                            }
                            if let Some((_, second)) = outer_portals.get_mut(&(ch1, ch2)) {
                                *second = portal;
                            } else {
                                outer_portals.insert((ch1, ch2), ((0, 0), portal));
                            }
                        }
                    }
                }
            }
        }
    }
    let start = outer_portals
        .remove(&('A', 'A'))
        .expect("Failed to read AA")
        .0;
    let end = outer_portals
        .remove(&('Z', 'Z'))
        .expect("Failed to read ZZ")
        .0;
    inner_portals.remove(&('A', 'A'));
    inner_portals.remove(&('Z', 'Z'));
    let outer_portals = outer_portals.iter().map(|(_, points)| *points).collect();
    let inner_portals = inner_portals.iter().map(|(_, points)| *points).collect();
    (map, outer_portals, inner_portals, start, end)
}

// this is dijkstra's
#[aoc(day20, part1)]
pub fn solve_day20_part1(
    (map, outer_portals, inner_portals, start, end): &(
        HashSet<Point>,
        HashMap<Point, Point>,
        HashMap<Point, Point>,
        Point,
        Point,
    ),
) -> usize {
    // Map of node to distance
    let mut distances: HashMap<Point, usize> =
        map.iter().map(|point| (*point, std::usize::MAX)).collect();
    distances.insert(*start, 0);
    while !distances.is_empty() {
        let (&current, &current_dist) = distances.iter().min_by_key(|(_, d)| *d).unwrap();
        if current == *end {
            return current_dist;
        }
        distances.remove(&current);
        let mut neighbors = vec![
            (current.0 + 1, current.1),
            (current.0 - 1, current.1),
            (current.0, current.1 + 1),
            (current.0, current.1 - 1),
        ];
        if let Some(portal_target) = outer_portals.get(&current) {
            neighbors.push(*portal_target);
        }
        if let Some(portal_target) = inner_portals.get(&current) {
            neighbors.push(*portal_target);
        }
        for neighbor in neighbors.iter() {
            distances.entry(*neighbor).and_modify(|neighbor_dist| {
                let dist = current_dist + 1;
                if dist < *neighbor_dist {
                    *neighbor_dist = dist;
                }
            });
        }
    }
    // Indicates failure
    0
}

// this is dijkstra's
#[aoc(day20, part2)]
pub fn solve_day20_part2(
    (map, outer_portals, inner_portals, start, end): &(
        HashSet<Point>,
        HashMap<Point, Point>,
        HashMap<Point, Point>,
        Point,
        Point,
    ),
) -> usize {
    // Map of node to distance
    let mut distances: HashMap<(usize, usize, usize), usize> = map
        .iter()
        .map(|point| ((point.0, point.1, 0), std::usize::MAX))
        .collect();
    let mut depth = 0;
    distances.insert((start.0, start.1, 0), 0);
    while !distances.is_empty() {
        let (&current, &current_dist) = distances.iter().min_by_key(|(_, d)| *d).unwrap();
        if current == (end.0, end.1, 0) {
            return current_dist;
        }
        distances.remove(&current);
        let mut neighbors = vec![
            (current.0 + 1, current.1, current.2),
            (current.0 - 1, current.1, current.2),
            (current.0, current.1 + 1, current.2),
            (current.0, current.1 - 1, current.2),
        ];
        // Outer portals dont work on the topmost level
        if current.2 > 0 {
            if let Some(portal_target) = outer_portals.get(&(current.0, current.1)) {
                neighbors.push((portal_target.0, portal_target.1, current.2 - 1));
            }
        }
        // Limit to depth 20
        if depth < 100 {
            if let Some(portal_target) = inner_portals.get(&(current.0, current.1)) {
                neighbors.push((portal_target.0, portal_target.1, current.2 + 1));
                // If needed add a layer to the map
                if depth < current.2 + 1 {
                    depth += 1;
                    for point in map.iter() {
                        distances.insert((point.0, point.1, depth), std::usize::MAX);
                    }
                }
            }
        }
        for neighbor in neighbors.iter() {
            distances.entry(*neighbor).and_modify(|neighbor_dist| {
                let dist = current_dist + 1;
                if dist < *neighbor_dist {
                    *neighbor_dist = dist;
                }
            });
        }
    }
    // Indicates failure
    0
}
