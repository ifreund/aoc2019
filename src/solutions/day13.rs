extern crate pancurses;
use crate::intcode;
use aoc_runner_derive::{aoc, aoc_generator};
use std::sync::mpsc::{sync_channel, TryRecvError};
use std::thread;
use std::time::Duration;

#[aoc_generator(day13)]
pub fn parse_day13(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

#[derive(Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Tile {
    fn symbol(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => 'X',
            Tile::HorizontalPaddle => '=',
            Tile::Ball => 'O',
        }
    }
}

#[aoc(day13, part1)]
pub fn solve_day13_part1(input: &[i64]) -> usize {
    let (_tx_in, rx_in) = sync_channel(20);
    let (tx_out, rx_out) = sync_channel(20);
    let program = input.to_vec();
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, None));

    let mut screen = vec![vec![Tile::Empty; 100]; 100];
    for x in &rx_out {
        let y = rx_out
            .recv()
            .expect("draw instructions must always have 3 values");
        let tile = match rx_out
            .recv()
            .expect("draw instructions must always have 3 values")
        {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("invalid tile"),
        };
        screen[y as usize][x as usize] = tile;
    }
    screen.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|tile| **tile == Tile::Block).count()
    })
}

#[aoc(day13, part2)]
pub fn solve_day13_part2(input: &[i64]) -> i64 {
    let (tx_in, rx_in) = sync_channel(0);
    let (tx_out, rx_out) = sync_channel(0);
    let (tx_req, rx_req) = sync_channel(0);
    let mut program = input.to_vec();
    // Insert two quarters
    program[0] = 2;
    thread::spawn(move || intcode::execute_threaded(program, rx_in, tx_out, Some(&tx_req)));

    let window = pancurses::initscr();
    pancurses::curs_set(0);
    pancurses::noecho();
    pancurses::start_color();
    pancurses::init_pair(0, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
    pancurses::init_pair(1, pancurses::COLOR_RED, pancurses::COLOR_BLACK);
    pancurses::init_pair(2, pancurses::COLOR_BLUE, pancurses::COLOR_BLACK);
    pancurses::init_pair(3, pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK);

    let mut score = 0;
    window.mvaddstr(25, 0, format!("{:#^1$}", format!(" SCORE: {} ", score), 42));
    let mut ballx = 0_i64;
    let mut paddlex = 24_i64;
    loop {
        match rx_req.try_recv() {
            Ok(()) => {
                tx_in
                    .send((ballx - paddlex).signum())
                    .expect("ERROR: failed to send input");
                window.refresh();
                thread::sleep(Duration::from_millis(10));
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        };
        if let Ok(x) = rx_out.try_recv() {
            let y = rx_out
                .recv()
                .expect("draw instructions must always have 3 values");
            let z = rx_out
                .recv()
                .expect("draw instructions must always have 3 values");

            match z {
                0 | 1 | 2 | 3 | 4 => {
                    let tile = match z {
                        0 => Tile::Empty,
                        1 => {
                            window.attrset(pancurses::COLOR_PAIR(0));
                            Tile::Wall
                        }
                        2 => {
                            window.attrset(pancurses::COLOR_PAIR(2));
                            Tile::Block
                        }
                        3 => {
                            window.attrset(pancurses::COLOR_PAIR(3));
                            paddlex = x;
                            Tile::HorizontalPaddle
                        }
                        4 => {
                            window.attrset(pancurses::COLOR_PAIR(1));
                            ballx = x;
                            Tile::Ball
                        }
                        _ => unreachable!(),
                    };
                    window.mvaddch(y as i32, x as i32, tile.symbol());
                }
                new_score if (x, y) == (-1, 0) => {
                    score = new_score;
                    window.attrset(pancurses::COLOR_PAIR(0));
                    window.mvaddstr(25, 0, format!("{:#^1$}", format!(" SCORE: {} ", score), 42));
                }
                _ => panic!("Invalid draw instruction"),
            };
        }
    }
    pancurses::endwin();
    score
}
