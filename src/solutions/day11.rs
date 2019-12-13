use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::TryRecvError;
use std::thread;

#[aoc_generator(day11)]
pub fn parse_day11(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

enum Rotation {
    Up,
    Down,
    Left,
    Right,
}

#[aoc(day11, part1)]
pub fn solve_day11_part1(program: &[i64]) -> usize {
    // Start the brain of the robot
    let (tx_in, rx_in) = sync_channel(0);
    let (tx_out, rx_out) = sync_channel(0);
    let (tx_req, rx_req) = sync_channel(0);
    let program = program.to_vec();
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, Some(&tx_req)));

    let mut bot_x = 0;
    let mut bot_y = 0;
    let mut bot_facing = Rotation::Up;
    let mut hull = HashMap::new();
    loop {
        match rx_req.try_recv() {
            Ok(()) => tx_in
                .send(*hull.get(&(bot_x, bot_y)).unwrap_or(&0))
                .expect("ERROR: failed to send color"),
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => return hull.len(),
        }
        if let Ok(color) = rx_out.try_recv() {
            hull.insert((bot_x, bot_y), color);
            let rotation = rx_out.recv().expect("ERROR: failed to recieve rotation");
            // Rotate according to brain output
            bot_facing = match (rotation, bot_facing) {
                (0, Rotation::Up) => Rotation::Left,
                (0, Rotation::Down) => Rotation::Right,
                (0, Rotation::Left) => Rotation::Down,
                (0, Rotation::Right) => Rotation::Up,
                (1, Rotation::Up) => Rotation::Right,
                (1, Rotation::Down) => Rotation::Left,
                (1, Rotation::Left) => Rotation::Up,
                (1, Rotation::Right) => Rotation::Down,
                _ => panic!("ERROR: invalid rotation"),
            };
            // Now move 1 forward
            match bot_facing {
                Rotation::Up => bot_y += 1,
                Rotation::Down => bot_y -= 1,
                Rotation::Left => bot_x -= 1,
                Rotation::Right => bot_x += 1,
            };
        }
    }
}

#[aoc(day11, part2)]
pub fn solve_day11_part2(program: &[i64]) -> String {
    // Start the brain of the robot
    let (tx_in, rx_in) = sync_channel(0);
    let (tx_out, rx_out) = sync_channel(0);
    let (tx_req, rx_req) = sync_channel(0);
    let program = program.to_vec();
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, Some(&tx_req)));

    let mut first = true;
    let mut bot_x = 0_i32;
    let mut bot_y = 0;
    let mut bot_facing = Rotation::Up;
    let mut hull = HashMap::new();
    loop {
        match rx_req.try_recv() {
            Ok(()) => tx_in
                .send(*hull.get(&(bot_x, bot_y)).unwrap_or(if first {
                    first = false;
                    &1
                } else {
                    &0
                }))
                .expect("ERROR: failed to send color"),
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
        if let Ok(color) = rx_out.try_recv() {
            hull.insert((bot_x, bot_y), color);
            let rotation = rx_out.recv().expect("ERROR: failed to recieve rotation");
            // Rotate according to brain output
            bot_facing = match (rotation, bot_facing) {
                (0, Rotation::Up) => Rotation::Left,
                (0, Rotation::Down) => Rotation::Right,
                (0, Rotation::Left) => Rotation::Down,
                (0, Rotation::Right) => Rotation::Up,
                (1, Rotation::Up) => Rotation::Right,
                (1, Rotation::Down) => Rotation::Left,
                (1, Rotation::Left) => Rotation::Up,
                (1, Rotation::Right) => Rotation::Down,
                _ => panic!("ERROR: invalid rotation"),
            };
            // Now move 1 forward
            match bot_facing {
                Rotation::Up => bot_y += 1,
                Rotation::Down => bot_y -= 1,
                Rotation::Left => bot_x -= 1,
                Rotation::Right => bot_x += 1,
            };
        }
    }
    let off_x = 0 - *hull
        .iter()
        .min_by_key(|((x, _), _)| x)
        .map(|((x, _), _)| x)
        .unwrap();
    let off_y = 0 - *hull
        .iter()
        .min_by_key(|((_, y), _)| y)
        .map(|((_, y), _)| y)
        .unwrap();

    let max_x = 1
        + off_x
        + *hull
            .iter()
            .max_by_key(|((x, _), _)| x)
            .map(|((x, _), _)| x)
            .unwrap();
    let max_y = 1
        + off_y
        + *hull
            .iter()
            .max_by_key(|((_, y), _)| y)
            .map(|((_, y), _)| y)
            .unwrap();

    let mut hull_paintjob = vec![vec![0; max_x as usize]; max_y as usize];

    for ((x, y), color) in hull.iter() {
        hull_paintjob[(*y + off_y) as usize][(*x + off_x) as usize] = *color;
    }

    hull_paintjob
        .iter()
        .rev()
        .map(|line| {
            line.iter()
                .map(|color| match color {
                    1 => '█',
                    _ => ' ',
                })
                .collect::<String>()
        })
        .fold("\n".to_owned(), |mut acc, row| {
            acc.push_str(&row);
            acc.push('\n');
            acc
        })
}
