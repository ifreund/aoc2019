extern crate pancurses;
extern crate rand;
use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};
use rand::Rng;
use std::sync::mpsc::{sync_channel, TryRecvError};
use std::thread;

#[aoc_generator(day15)]
pub fn parse_day13(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

//#[aoc(day15, part1, manual)]
#[allow(dead_code)]
pub fn solve_day15_part1_manual(input: &[i64]) -> usize {
    let (tx_in, rx_in) = sync_channel(0);
    let (tx_out, rx_out) = sync_channel(0);
    let (tx_req, rx_req) = sync_channel(0);
    let program = input.to_vec();
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, Some(&tx_req)));

    let window = pancurses::initscr();
    pancurses::curs_set(0);
    pancurses::noecho();

    let (mut x, mut y) = (25, 25);
    let mut last_move = 0;
    loop {
        match rx_req.try_recv() {
            Ok(()) => {
                tx_in
                    .send({
                        last_move = 0;
                        while last_move == 0 {
                            last_move = match window.getch() {
                                Some(pancurses::Input::Character('k')) => 1, // North
                                Some(pancurses::Input::Character('j')) => 2, // South
                                Some(pancurses::Input::Character('h')) => 3, // West
                                Some(pancurses::Input::Character('l')) => 4, // East
                                _ => 0,
                            };
                        }
                        last_move
                    })
                    .expect("ERROR: failed to send input");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        };
        match rx_out.try_recv() {
            Ok(0) => {
                match last_move {
                    1 => window.mvaddch(y - 1, x, '#'),
                    2 => window.mvaddch(y + 1, x, '#'),
                    3 => window.mvaddch(y, x - 1, '#'),
                    4 => window.mvaddch(y, x + 1, '#'),
                    _ => unreachable!(),
                };
                window.refresh();
            }
            Ok(1) => {
                let (newx, newy) = match last_move {
                    1 => (x, y - 1),
                    2 => (x, y + 1),
                    3 => (x - 1, y),
                    4 => (x + 1, y),
                    _ => unreachable!(),
                };
                window.mvaddch(newy, newx, '@');
                window.mvaddch(y, x, '.');
                window.refresh();
                x = newx;
                y = newy;
            }
            Ok(2) => {
                let (newx, newy) = match last_move {
                    1 => (x, y - 1),
                    2 => (x, y + 1),
                    3 => (x - 1, y),
                    4 => (x + 1, y),
                    _ => unreachable!(),
                };
                window.mvaddch(newx, newy, 'O');
                window.mvaddch(y, x, '.');
                window.refresh();
                x = newx;
                y = newy;
            }
            _ => (),
        }
    }
    pancurses::endwin();
    777
}

#[derive(PartialEq, Clone)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    Oxygen,
}

fn get_next_move(map: &[Vec<Tile>], (x, y): (usize, usize)) -> i64 {
    if map[y - 1][x] == Tile::Unknown {
        return 1;
    }
    if map[y + 1][x] == Tile::Unknown {
        return 2;
    }
    if map[y][x - 1] == Tile::Unknown {
        return 3;
    }
    if map[y][x + 1] == Tile::Unknown {
        return 4;
    }
    // This is of course horribly inefficient, but still solves the maze in about 5 seconds and is
    // very simple
    rand::thread_rng().gen_range(1, 5)
}

// this is dijkstra's
fn shortest_path(map: &[Vec<Tile>], target: (usize, usize)) -> usize {
    // Map of unvisited node to distance
    let mut distances = std::collections::HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Empty || *tile == Tile::Oxygen {
                distances.insert((x, y), std::usize::MAX);
            }
        }
    }
    distances.insert((25, 25), 0);
    while !distances.is_empty() {
        let (&current, &current_dist) = distances.iter().min_by_key(|(_, d)| *d).unwrap();
        distances.remove(&current);
        if current == target {
            return current_dist;
        }
        for neighbor in [
            (current.0 + 1, current.1),
            (current.0 - 1, current.1),
            (current.0, current.1 + 1),
            (current.0, current.1 - 1),
        ]
        .iter()
        {
            distances.entry(*neighbor).and_modify(|neighbor_dist| {
                *neighbor_dist = std::cmp::min(*neighbor_dist, current_dist + 1)
            });
        }
    }
    panic!("Couldn't reach target");
}

fn explore_map(input: &[i64]) -> (Vec<Vec<Tile>>, (usize, usize)) {
    let (tx_in, rx_in) = sync_channel(0);
    let (tx_out, rx_out) = sync_channel(0);
    let (tx_req, rx_req) = sync_channel(0);
    let program = input.to_vec();
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, Some(&tx_req)));

    let mut map = vec![vec![Tile::Unknown; 50]; 50];

    let (mut x, mut y) = (25, 25);
    map[y][x] = Tile::Empty;
    let mut new_move = 0;

    let get_new_coords = |new_move, x, y| match new_move {
        1 => (x, y - 1),
        2 => (x, y + 1),
        3 => (x - 1, y),
        4 => (x + 1, y),
        _ => unreachable!(),
    };
    loop {
        if rx_req.try_recv().is_ok() {
            tx_in
                .send({
                    new_move = get_next_move(&map, (x, y));
                    new_move
                })
                .expect("ERROR: failed to send input");
        }
        match rx_out.try_recv() {
            Ok(0) => {
                let (newx, newy) = get_new_coords(new_move, x, y);
                map[newy][newx] = Tile::Wall;
            }
            Ok(1) => {
                let (newx, newy) = get_new_coords(new_move, x, y);
                map[newy][newx] = Tile::Empty;
                x = newx;
                y = newy;
            }
            Ok(2) => {
                let (newx, newy) = get_new_coords(new_move, x, y);
                map[newy][newx] = Tile::Oxygen;
                return (map, (newx, newy));
            }
            _ => (),
        }
    }
}

#[aoc(day15, part1)]
pub fn solve_day15_part1(input: &[i64]) -> usize {
    let (map, oxy) = explore_map(input);
    shortest_path(&map, oxy)
}

// this is dijkstra's
fn longest_path(map: &[Vec<Tile>], start: (usize, usize)) -> usize {
    // Map of unvisited node to distance
    let mut distances = std::collections::HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Empty || *tile == Tile::Oxygen {
                distances.insert((x, y), std::usize::MAX);
            }
        }
    }
    distances.insert(start, 0);
    let mut max_dist = 0;
    while !distances.is_empty() {
        let (&current, &current_dist) = distances.iter().min_by_key(|(_, d)| *d).unwrap();
        max_dist = std::cmp::max(max_dist, current_dist);
        distances.remove(&current);
        for neighbor in [
            (current.0 + 1, current.1),
            (current.0 - 1, current.1),
            (current.0, current.1 + 1),
            (current.0, current.1 - 1),
        ]
        .iter()
        {
            distances.entry(*neighbor).and_modify(|neighbor_dist| {
                *neighbor_dist = std::cmp::min(*neighbor_dist, current_dist + 1)
            });
        }
    }
    max_dist
}

#[aoc(day15, part2)]
pub fn solve_day15_part2(input: &[i64]) -> usize {
    let (map, oxy) = explore_map(input);
    longest_path(&map, oxy)
}
