use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;
use std::sync::mpsc::sync_channel;
use std::thread;

#[aoc_generator(day21)]
pub fn parse_day21(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

//#[aoc(day21, part1, interactive)]
#[allow(dead_code)]
pub fn solve_day21_part1_interactive(program: &[i64]) -> i64 {
    let (tx_in, rx_in) = sync_channel(0);
    let (tx_out, rx_out) = sync_channel(0);
    let (tx_req, rx_req) = sync_channel(0);
    let program = program.to_vec();
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, Some(&tx_req)));

    let mut input_buffer = VecDeque::new();
    loop {
        match rx_req.try_recv() {
            Ok(()) => {
                tx_in
                    .send({
                        if let Some(ch) = input_buffer.pop_front() {
                            ch as i64
                        } else {
                            let mut input = String::new();
                            std::io::stdin().read_line(&mut input).unwrap();
                            for ch in input.chars() {
                                input_buffer.push_back(ch);
                            }
                            input_buffer.pop_front().unwrap() as i64
                        }
                    })
                    .expect("ERROR: failed to send input");
            }
            _ => (),
        };
        match rx_out.try_recv() {
            Ok(out) if 0 <= out && out < 128 => {
                print!("{}", std::char::from_u32(out as u32).unwrap());
            }
            Ok(out) => return out,
            _ => (),
        }
    }
}

#[aoc(day21, part1, auto)]
pub fn solve_day21_part1_auto(program: &[i64]) -> i64 {
    const SPRING_SCRIPT: &str = "NOT J J\n\
                                 AND A J\n\
                                 AND B J\n\
                                 AND C J\n\
                                 NOT J J\n\
                                 AND D J\n\
                                 WALK\n";
    let (tx_in, rx_in) = sync_channel(0);
    let (tx_out, rx_out) = sync_channel(5000);
    let (tx_req, rx_req) = sync_channel(0);
    let program = program.to_vec();
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, Some(&tx_req)));

    for ch in SPRING_SCRIPT.chars() {
        rx_req.recv().unwrap();
        tx_in.send(ch as i64).unwrap();
    }
    rx_out.iter().last().unwrap()
}

#[aoc(day21, part2, auto)]
pub fn solve_day21_part2_auto(program: &[i64]) -> i64 {
    const SPRING_SCRIPT: &str = "NOT J J\n\
                                 AND A J\n\
                                 AND B J\n\
                                 AND C J\n\
                                 NOT J J\n\
                                 AND D J\n\
                                 OR H T\n\
                                 OR E T\n\
                                 AND T J \n\
                                 RUN\n";
    let (tx_in, rx_in) = sync_channel(0);
    let (tx_out, rx_out) = sync_channel(5000);
    let (tx_req, rx_req) = sync_channel(0);
    let program = program.to_vec();
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, Some(&tx_req)));

    for ch in SPRING_SCRIPT.chars() {
        rx_req.recv().unwrap();
        tx_in.send(ch as i64).unwrap();
    }
    rx_out.iter().last().unwrap()
}
