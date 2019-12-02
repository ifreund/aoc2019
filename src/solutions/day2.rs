use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn parse_day2(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn compute(memory: &[usize], noun: usize, verb: usize) -> usize {
    let mut memory = memory.to_owned();

    memory[1] = noun;
    memory[2] = verb;

    let mut i = 0;
    loop {
        match memory[i] {
            1 => {
                let read_1 = memory[i + 1];
                let read_2 = memory[i + 2];
                let write = memory[i + 3];
                memory[write] = memory[read_1] + memory[read_2];
            }
            2 => {
                let read_1 = memory[i + 1];
                let read_2 = memory[i + 2];
                let write = memory[i + 3];
                memory[write] = memory[read_1] * memory[read_2];
            }
            99 => return memory[0],
            _ => panic!("ERROR: {} is not a valid opcode", memory[i]),
        }
        i += 4;
    }
}

#[aoc(day2, part1)]
pub fn solve_day2_part1(input: &[usize]) -> usize {
    compute(input, 12, 2)
}

#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &[usize]) -> usize {
    const TARGET: usize = 19690720;
    (0..100)
        .flat_map(|noun| (0..100).map(move |verb| (noun, verb)))
        .find(|(noun, verb)| compute(input, *noun, *verb) == TARGET)
        .map(|(noun, verb)| 100 * noun + verb)
        .unwrap()
}
