use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};
use std::sync::mpsc::sync_channel;
use std::thread;

#[aoc_generator(day17)]
pub fn parse_day17(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[derive(PartialEq)]
enum Tile {
    Scaffold,
    Empty,
    BotNorth,
    BotSouth,
    BotEast,
    BotWest,
}

#[aoc(day17, part1)]
pub fn solve_day17_part1(input: &[i64]) -> usize {
    let (_, rx_in) = sync_channel(0);
    let (tx_out, rx_out) = sync_channel(0);
    let program = input.to_vec();
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, None));

    let mut view = Vec::new();
    view.push(Vec::new());
    for ch in rx_out {
        match ch as u8 {
            b'#' => view.last_mut().unwrap().push(Tile::Scaffold),
            b'.' => view.last_mut().unwrap().push(Tile::Empty),
            b'^' => view.last_mut().unwrap().push(Tile::BotNorth),
            b'v' => view.last_mut().unwrap().push(Tile::BotSouth),
            b'>' => view.last_mut().unwrap().push(Tile::BotEast),
            b'<' => view.last_mut().unwrap().push(Tile::BotWest),
            b'\n' => view.push(Vec::new()),
            _ => panic!("ERROR: {} is not a known char", ch),
        }
    }
    // get rid of the extra newlines
    view.pop();
    view.pop();

    let mut intersections = std::collections::HashSet::new();
    for y in 1..(view.len() - 1) {
        for x in 1..(view.first().unwrap().len() - 1) {
            if view[y - 1][x] != Tile::Empty
                && view[y + 1][x] != Tile::Empty
                && view[y][x - 1] != Tile::Empty
                && view[y][x + 1] != Tile::Empty
            {
                intersections.insert((x, y));
            }
        }
    }

    for (y, row) in view.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            print!(
                "{}",
                match tile {
                    _ if intersections.contains(&(x, y)) => 'O',
                    Tile::Scaffold => '#',
                    Tile::Empty => '.',
                    Tile::BotNorth => '^',
                    Tile::BotSouth => 'v',
                    Tile::BotEast => '>',
                    Tile::BotWest => '<',
                }
            );
        }
        println!();
    }
    intersections.iter().map(|(x, y)| x * y).sum()
}

// Entire path:
// R,10,L,12,R,6,R,10,L,12,R,6,R,6,R,10,R,12,R,6,R,10,L,12,L,12,R,6,R,10,R,12,R,6,
// R,10,L,12,L,12,R,6,R,10,R,12,R,6,R,10,L,12,L,12,R,6,R,10,R,12,R,6,R,10,L,12,R,6

const MAIN: &str = "C,C,A,B,A,B,A,B,A,C\n";
const A: &str = "R,6,R,10,R,12,R,6\n";
const B: &str = "R,10,L,12,L,12\n";
const C: &str = "R,10,L,12,R,6\n";

#[aoc(day17, part2)]
fn solve_day17_part2(input: &[i64]) -> i64 {
    let (tx_in, rx_in) = sync_channel(0);
    // All we care about is the very last output, so don't block
    let (tx_out, rx_out) = sync_channel(5000);
    let (tx_req, rx_req) = sync_channel(0);
    let mut program = input.to_vec();
    // Start the vacuum robot
    program[0] = 2;
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, Some(&tx_req)));

    for byte in MAIN
        .as_bytes()
        .iter()
        .chain(A.as_bytes().iter())
        .chain(B.as_bytes().iter())
        .chain(C.as_bytes().iter())
        .chain(b"n\n".iter())
    {
        rx_req.recv().unwrap();
        tx_in.send(*byte as i64).unwrap();
    }

    rx_out.iter().last().unwrap()
}
