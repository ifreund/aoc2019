use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[derive(PartialEq)]
enum Opcode {
    Add,
    Mult,
    In,
    Out,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

enum Mode {
    Position,
    Immediate,
}

struct Instruction {
    opcode: Opcode,
    parameter_modes: Vec<Mode>,
}

impl Instruction {
    fn new(raw: i32) -> Self {
        // ABCDE
        //  1002
        //
        // DE - two-digit opcode,      02 == opcode 2
        //  C - mode of 1st parameter,  0 == position mode
        //  B - mode of 2nd parameter,  1 == immediate mode
        //  A - mode of 3rd parameter,  0 == position mode,
        //                                  omitted due to being a leading zero
        let opcode = match raw % 100 {
            1 => Opcode::Add,
            2 => Opcode::Mult,
            3 => Opcode::In,
            4 => Opcode::Out,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            _ => panic!("ERROR: {} has an invaild opcode", raw),
        };
        let parameter_modes = {
            fn read_mode(raw_mode: i32) -> Mode {
                match raw_mode {
                    0 => Mode::Position,
                    1 => Mode::Immediate,
                    _ => panic!("ERROR: {} is an invalid parameter mode", raw_mode),
                }
            }
            let param1 = read_mode(raw % 1000 / 100);
            let param2 = read_mode(raw % 10_000 / 1000);
            let param3 = read_mode(raw % 100_000 / 10_000);
            vec![param1, param2, param3]
        };
        Instruction {
            opcode,
            parameter_modes,
        }
    }
}

fn read_param(
    param_idx: usize,
    instruction: &Instruction,
    instruction_pointer: usize,
    memory: &[i32],
) -> i32 {
    let param = memory[instruction_pointer + param_idx];
    match instruction.parameter_modes[param_idx - 1] {
        Mode::Position => memory[param as usize],
        Mode::Immediate => param,
    }
}

pub fn execute(program: &[i32], input: &[i32]) -> i32 {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();
    let program = program.to_vec();
    thread::spawn(move || execute_threaded(program, rx_in, tx_out));
    for value in input {
        tx_in.send(*value).unwrap();
    }
    rx_out.iter().last().unwrap()
}

pub fn execute_threaded(mut memory: Vec<i32>, input: Receiver<i32>, output: Sender<i32>) {
    let mut instruction_pointer = 0;
    loop {
        let instruction = Instruction::new(memory[instruction_pointer]);
        match instruction.opcode {
            Opcode::Add | Opcode::Mult | Opcode::LessThan | Opcode::Equals => {
                let param1 = read_param(1, &instruction, instruction_pointer, &memory);
                let param2 = read_param(2, &instruction, instruction_pointer, &memory);
                let addr = memory[instruction_pointer + 3];
                memory[addr as usize] = match instruction.opcode {
                    Opcode::Add => param1 + param2,
                    Opcode::Mult => param1 * param2,
                    Opcode::LessThan => {
                        if param1 < param2 {
                            1
                        } else {
                            0
                        }
                    }
                    Opcode::Equals => {
                        if param1 == param2 {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                };
                instruction_pointer += 4;
            }
            Opcode::In => {
                let addr = memory[instruction_pointer + 1];
                memory[addr as usize] = input.recv().expect("ERROR: failed to receive input");
                instruction_pointer += 2;
            }
            Opcode::Out => {
                output
                    .send(read_param(1, &instruction, instruction_pointer, &memory))
                    .expect("ERROR: failed to send output");
                instruction_pointer += 2;
            }
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => {
                let param1 = read_param(1, &instruction, instruction_pointer, &memory);
                let param2 = read_param(2, &instruction, instruction_pointer, &memory);
                if param1 != 0 && instruction.opcode == Opcode::JumpIfTrue
                    || param1 == 0 && instruction.opcode == Opcode::JumpIfFalse
                {
                    instruction_pointer = param2 as usize;
                } else {
                    instruction_pointer += 3;
                }
            }
            Opcode::Halt => return,
        }
    }
}
