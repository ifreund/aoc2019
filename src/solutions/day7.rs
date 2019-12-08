use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;
use std::sync::mpsc::channel;
use std::thread;

#[aoc_generator(day7)]
pub fn parse_day7(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn next_permutation(current: &mut [i32]) -> Option<&[i32]> {
    let j = (0..current.len())
        .rev()
        .skip(1)
        .find(|i| current[*i] < current[*i + 1])?;
    let k = current.iter().rposition(|i| current[j] < *i).unwrap();
    current.swap(j, k);
    current[j + 1..].reverse();
    Some(current)
}

#[aoc(day7, part1)]
pub fn solve_day7_part1(program: &[i32]) -> i32 {
    let mut phase_settings = vec![0, 1, 2, 3, 4];
    let mut current_best = 0;
    while let Some(permutation) = next_permutation(phase_settings.as_mut_slice()) {
        let mut signal = 0;
        for setting in permutation.iter() {
            signal = intcode::execute(program, &[*setting, signal]);
        }
        current_best = max(current_best, signal);
    }
    current_best
}

#[aoc(day7, part2)]
pub fn solve_day7_part2(program: &[i32]) -> i32 {
    let mut phase_settings = vec![5, 6, 7, 8, 9];
    let mut current_best = 0;
    while let Some(permutation) = next_permutation(phase_settings.as_mut_slice()) {
        let (tx_ma, rx_ma) = channel();
        let (tx_ab, rx_ab) = channel();
        let (tx_bc, rx_bc) = channel();
        let (tx_cd, rx_cd) = channel();
        let (tx_de, rx_de) = channel();
        let (tx_em, rx_em) = channel();

        // Initalize amplifiers
        tx_ma.send(permutation[0]).unwrap();
        tx_ab.send(permutation[1]).unwrap();
        tx_bc.send(permutation[2]).unwrap();
        tx_cd.send(permutation[3]).unwrap();
        tx_de.send(permutation[4]).unwrap();

        // Spawn intcode processors
        let cloned = program.to_vec();
        thread::spawn(move || intcode::execute_threaded(cloned, rx_ma, tx_ab));
        let cloned = program.to_vec();
        thread::spawn(move || intcode::execute_threaded(cloned, rx_ab, tx_bc));
        let cloned = program.to_vec();
        thread::spawn(move || intcode::execute_threaded(cloned, rx_bc, tx_cd));
        let cloned = program.to_vec();
        thread::spawn(move || intcode::execute_threaded(cloned, rx_cd, tx_de));
        let cloned = program.to_vec();
        thread::spawn(move || intcode::execute_threaded(cloned, rx_de, tx_em));

        // Start the amplification
        tx_ma.send(0).unwrap();

        let mut thruster_signal = 0;
        for signal in rx_em {
            thruster_signal = signal;
            tx_ma.send(signal).ok();
        }
        current_best = max(current_best, thruster_signal);
    }
    current_best
}
