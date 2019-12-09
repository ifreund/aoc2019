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
    AdjustRelativeBase,
    Halt,
}

enum Mode {
    Position,
    Immediate,
    Relative,
}

struct Instruction {
    opcode: Opcode,
    parameter_modes: Vec<Mode>,
}

impl Instruction {
    fn new(raw: i64) -> Self {
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
            9 => Opcode::AdjustRelativeBase,
            99 => Opcode::Halt,
            _ => panic!("ERROR: {} has an invaild opcode", raw),
        };
        let parameter_modes = {
            fn read_mode(raw_mode: i64) -> Mode {
                match raw_mode {
                    0 => Mode::Position,
                    1 => Mode::Immediate,
                    2 => Mode::Relative,
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

pub fn execute(program: &[i64], input: &[i64]) -> Vec<i64> {
    let (tx_in, rx_in) = channel();
    let (tx_out, rx_out) = channel();
    let program = program.to_vec();
    thread::spawn(move || execute_threaded(program, rx_in, tx_out));
    for value in input {
        tx_in.send(*value).unwrap();
    }
    rx_out.iter().collect()
}

fn read_param(
    param_idx: usize,
    instruction: &Instruction,
    instruction_pointer: usize,
    memory: &[i64],
    relative_base: i64,
) -> i64 {
    let param = memory[instruction_pointer + param_idx];
    match instruction.parameter_modes[param_idx - 1] {
        Mode::Position => memory[param as usize],
        Mode::Immediate => param,
        Mode::Relative => memory[(param + relative_base) as usize],
    }
}

fn write_param(
    value: i64,
    param_idx: usize,
    instruction: &Instruction,
    instruction_pointer: usize,
    memory: &mut [i64],
    relative_base: i64,
) {
    let param = memory[instruction_pointer + param_idx];
    match instruction.parameter_modes[param_idx - 1] {
        Mode::Position => memory[param as usize] = value,
        Mode::Immediate => panic!("ERROR: cannnot write to immediate parameter"),
        Mode::Relative => memory[(relative_base + param) as usize] = value,
    };
}

pub fn execute_threaded(mut memory: Vec<i64>, input: Receiver<i64>, output: Sender<i64>) {
    // Expand and fill with zeros
    memory.resize(0xFFFF, 0);
    let mut instruction_pointer = 0;
    let mut relative_base = 0;
    loop {
        let instruction = Instruction::new(memory[instruction_pointer]);
        match instruction.opcode {
            Opcode::Add | Opcode::Mult | Opcode::LessThan | Opcode::Equals => {
                let param1 =
                    read_param(1, &instruction, instruction_pointer, &memory, relative_base);
                let param2 =
                    read_param(2, &instruction, instruction_pointer, &memory, relative_base);
                let value = match instruction.opcode {
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
                write_param(
                    value,
                    3,
                    &instruction,
                    instruction_pointer,
                    &mut memory,
                    relative_base,
                );
                instruction_pointer += 4;
            }
            Opcode::In => {
                let value = input.recv().expect("ERROR: failed to receive input");
                write_param(
                    value,
                    1,
                    &instruction,
                    instruction_pointer,
                    &mut memory,
                    relative_base,
                );
                instruction_pointer += 2;
            }
            Opcode::Out => {
                output
                    .send(read_param(
                        1,
                        &instruction,
                        instruction_pointer,
                        &memory,
                        relative_base,
                    ))
                    .expect("ERROR: failed to send output");
                instruction_pointer += 2;
            }
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => {
                let param1 =
                    read_param(1, &instruction, instruction_pointer, &memory, relative_base);
                let param2 =
                    read_param(2, &instruction, instruction_pointer, &memory, relative_base);
                if param1 != 0 && instruction.opcode == Opcode::JumpIfTrue
                    || param1 == 0 && instruction.opcode == Opcode::JumpIfFalse
                {
                    instruction_pointer = param2 as usize;
                } else {
                    instruction_pointer += 3;
                }
            }
            Opcode::AdjustRelativeBase => {
                relative_base +=
                    read_param(1, &instruction, instruction_pointer, &memory, relative_base);
                instruction_pointer += 2;
            }
            Opcode::Halt => return,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn intcode_test1() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_eq!(program, execute(&program, &[]));
    }

    #[test]
    fn intcode_test2() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        assert!(*execute(&program, &[]).last().unwrap() >= 1_000_000_000_000_000);
    }

    #[test]
    fn intcode_test3() {
        let program = vec![104, 1125899906842624, 99];
        assert_eq!(1125899906842624, *execute(&program, &[]).last().unwrap());
    }
}
