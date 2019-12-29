use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;
use std::sync::mpsc::sync_channel;
use std::thread;

#[aoc_generator(day25)]
pub fn parse_day25(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day25, part1)]
pub fn solve_day25_part1(program: &[i64]) -> i64 {
    const COMMANDS: &str = "north\n\
                            west\n\
                            take antenna\n\
                            south\n\
                            take hologram\n\
                            west\n\
                            take astronaut ice cream\n\
                            east\n\
                            north\n\
                            north\n\
                            north\n\
                            north\n\
                            take space heater\n\
                            north\n\
                            east\n\
                            east\n";
    let (tx_in, rx_in) = sync_channel(0);
    let (tx_out, rx_out) = sync_channel(0);
    let (tx_req, rx_req) = sync_channel(0);
    let program = program.to_vec();
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, Some(&tx_req)));

    let mut input_buffer = VecDeque::new();
    for ch in COMMANDS.chars() {
        input_buffer.push_back(ch);
    }
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
