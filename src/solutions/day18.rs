use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type Point = (i32, i32);

#[aoc_generator(day18)]
pub fn parse_day18(
    input: &str,
) -> (
    Point,
    HashSet<Point>,
    HashMap<Point, u32>,
    HashMap<Point, u32>,
) {
    let mut start = None;
    let mut keys = HashMap::new();
    let mut doors = HashMap::new();
    let mut walls = HashSet::new();
    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            match ch {
                '#' => {
                    walls.insert((x, y));
                }
                '@' => start = Some((x, y)),
                '.' => (),
                ch if ch.is_ascii_lowercase() => {
                    keys.insert((x, y), 1 << (ch as u8 - b'a'));
                }
                ch if ch.is_ascii_uppercase() => {
                    doors.insert((x, y), 1 << (ch.to_ascii_lowercase() as u8 - b'a'));
                }
                _ => panic!(),
            }
        }
    }
    (start.expect("Failed to find start"), walls, keys, doors)
}

#[derive(Eq)]
struct State {
    position: Point,
    distance: usize,
    keys: u32,
}

// Note that this is "backwards" turning the BinaryHeap into a min queue instead of a max queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

// This is dijkstra's
fn find_all_keys(
    start: Point,
    walls: &HashSet<Point>,
    keys: &HashMap<Point, u32>,
    doors: &HashMap<Point, u32>,
) -> Option<usize> {
    let key_dist_cache: HashMap<Point, Vec<(Point, usize, u32, u32)>> = keys
        .keys()
        .chain(std::iter::once(&start))
        .map(|point| (*point, get_distances(*point, &walls, &keys, &doors)))
        .collect();

    let mut discovered = HashSet::new();
    let mut pqueue = BinaryHeap::new();

    // 0 distance and 0 keys
    pqueue.push(State {
        position: start,
        distance: 0,
        keys: 0,
    });

    let all_keys = keys.values().fold(0, |acc, key| acc | key);

    while let Some(current) = pqueue.pop() {
        if current.keys == all_keys {
            return Some(current.distance);
        }
        if discovered.insert((current.position, current.keys)) {
            for (new_pos, cost, new_key, _) in key_dist_cache
                .get(&current.position)
                .unwrap()
                .iter()
                .filter(|(_, _, target_key, doors_needed)| {
                    // if we don't have the target key yet but do have the keys to open all the doors
                    current.keys & target_key == 0 && !current.keys & doors_needed == 0
                })
            {
                pqueue.push(State {
                    position: *new_pos,
                    distance: current.distance + cost,
                    keys: current.keys | new_key,
                });
            }
        }
    }
    None
}

// This is a bfs
fn get_distances(
    start: Point,
    walls: &HashSet<Point>,
    keys: &HashMap<Point, u32>,
    doors: &HashMap<Point, u32>,
) -> Vec<(Point, usize, u32, u32)> {
    let mut distances = Vec::new();
    let mut discovered = HashSet::new();
    let mut queue = VecDeque::new();

    // 0 distance and no doors seen
    queue.push_back((start, 0, 0));

    while let Some((cur_pos, cur_dist, cur_doors)) = queue.pop_front() {
        if discovered.insert(cur_pos) {
            if let Some(key) = keys.get(&cur_pos) {
                distances.push((cur_pos, cur_dist, *key, cur_doors));
            }
            for neighbor in [
                (cur_pos.0 + 1, cur_pos.1),
                (cur_pos.0 - 1, cur_pos.1),
                (cur_pos.0, cur_pos.1 + 1),
                (cur_pos.0, cur_pos.1 - 1),
            ]
            .iter()
            {
                if !walls.contains(neighbor) {
                    queue.push_back((
                        *neighbor,
                        cur_dist + 1,
                        cur_doors | doors.get(neighbor).unwrap_or(&0),
                    ));
                }
            }
        }
    }
    distances
}

#[aoc(day18, part1)]
pub fn solve_day18_part1(
    (start, walls, keys, doors): &(
        Point,
        HashSet<Point>,
        HashMap<Point, u32>,
        HashMap<Point, u32>,
    ),
) -> usize {
    find_all_keys(*start, &walls, &keys, &doors).expect("There is no path that obtains all keys")
}

#[aoc(day18, part2)]
pub fn solve_day18_part2(
    (start, walls, keys, doors): &(
        Point,
        HashSet<Point>,
        HashMap<Point, u32>,
        HashMap<Point, u32>,
    ),
) -> usize {
    let mut walls = walls.clone();
    // Modify the map as instructed
    for new_wall in [
        (start.0 + 1, start.1),
        (start.0 - 1, start.1),
        (start.0, start.1 + 1),
        (start.0, start.1 - 1),
    ]
    .iter()
    {
        walls.insert(*new_wall);
    }
    let walls = walls;

    [(1, 1), (1, -1), (-1, 1), (-1, -1)]
        .iter()
        .map(|(dx, dy)| {
            let new_keys = keys
                .iter()
                .filter(|((key_x, key_y), _)| {
                    dx * key_x > dx * start.0 && dy * key_y > dy * start.1
                })
                .map(|(k, v)| (*k, *v))
                .collect::<HashMap<_, _>>();
            let new_doors = doors
                .iter()
                .filter(|((key_x, key_y), _)| {
                    dx * key_x > dx * start.0 && dy * key_y > dy * start.1
                })
                .filter(|(_, key_needed)| {
                    new_keys.iter().find(|(_, key)| key == key_needed).is_some()
                })
                .map(|(k, v)| (*k, *v))
                .collect::<HashMap<_, _>>();
            find_all_keys((start.0 + dx, start.1 + dy), &walls, &new_keys, &new_doors)
                .expect("There is no path that obtains all keys")
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day18_example1() {
        const INPUT: &str = "########################\n\
                             #f.D.E.e.C.b.A.@.a.B.c.#\n\
                             ######################.#\n\
                             #d.....................#\n\
                             ########################";
        assert_eq!(solve_day18_part1(&parse_day18(INPUT)), 86);
    }

    #[test]
    fn day18_example2() {
        const INPUT: &str = "########################\n\
                             #...............b.C.D.f#\n\
                             #.######################\n\
                             #.....@.a.B.c.d.A.e.F.g#\n\
                             ########################";
        assert_eq!(solve_day18_part1(&parse_day18(INPUT)), 132);
    }

    #[test]
    fn day18_example3() {
        const INPUT: &str = "#################\n\
                             #i.G..c...e..H.p#\n\
                             ########.########\n\
                             #j.A..b...f..D.o#\n\
                             ########@########\n\
                             #k.E..a...g..B.n#\n\
                             ########.########\n\
                             #l.F..d...h..C.m#\n\
                             #################";
        assert_eq!(solve_day18_part1(&parse_day18(INPUT)), 136);
    }
}
