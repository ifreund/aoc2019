use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

#[aoc_generator(day23)]
pub fn parse_day23(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day23, part1)]
pub fn solve_day23_part1(program: &[i64]) -> i64 {
    let mut tx_ins = Vec::new();
    let mut rx_outs = Vec::new();

    for i in 0..50 {
        let (tx_in, rx_in) = channel();
        let (tx_out, rx_out) = channel();
        tx_in.send(i).unwrap();
        tx_ins.push(tx_in);
        rx_outs.push(rx_out);
        let program = program.to_vec();
        thread::spawn(move || intcode::execute_threaded_async(program, rx_in, tx_out));
    }

    loop {
        for i in 0..50 {
            if let Ok(dest) = rx_outs[i].try_recv() {
                if dest == 255 {
                    rx_outs[i].recv().unwrap(); // X
                    return rx_outs[i].recv().unwrap(); // Y
                }
                // Send X and Y to the destination address
                let x = rx_outs[i].recv().unwrap();
                let y = rx_outs[i].recv().unwrap();
                tx_ins[dest as usize].send(x).unwrap();
                tx_ins[dest as usize].send(y).unwrap();
            }
        }
    }
}

#[aoc(day23, part2)]
pub fn solve_day23_part2(program: &[i64]) -> i64 {
    let mut tx_ins = Vec::new();
    let mut rx_outs = Vec::new();

    for i in 0..50 {
        let (tx_in, rx_in) = channel();
        let (tx_out, rx_out) = channel();
        tx_in.send(i).unwrap();
        tx_ins.push(tx_in);
        rx_outs.push(rx_out);
        let program = program.to_vec();
        thread::spawn(move || intcode::execute_threaded_async(program, rx_in, tx_out));
    }
    let mut hist_y = HashSet::new();
    let mut nat_x = 0;
    let mut nat_y = 0;
    let mut idle_cycles = 0;
    loop {
        idle_cycles += 1;
        for i in 0..50 {
            if let Ok(dest) = rx_outs[i].try_recv() {
                idle_cycles = 0;
                if dest == 255 {
                    nat_x = rx_outs[i].recv().unwrap(); // X
                    nat_y = rx_outs[i].recv().unwrap(); // Y
                } else {
                    // Send X and Y to the destination address
                    let x = rx_outs[i].recv().unwrap();
                    let y = rx_outs[i].recv().unwrap();
                    tx_ins[dest as usize].send(x).unwrap();
                    tx_ins[dest as usize].send(y).unwrap();
                }
            }
        }
        // This allows the child threads time to process any input
        // Without this idle is detected spuriously
        // Note: This is by no means robust but works in this case
        thread::sleep(Duration::from_millis(1));
        if idle_cycles > 1000 {
            tx_ins[0].send(nat_x).unwrap();
            tx_ins[0].send(nat_y).unwrap();
            if hist_y.contains(&nat_y) {
                return nat_y;
            }
            hist_y.insert(nat_y);
            idle_cycles = 0;
        }
    }
}
