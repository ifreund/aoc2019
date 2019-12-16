use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day16)]
pub fn parse_day16(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .collect()
}

fn calc_phase(input: &[i32], pattern: &[i32]) -> Vec<i32> {
    (1..=input.len())
        .map(|i| {
            let cur_pat = pattern
                .iter()
                .flat_map(|x| std::iter::repeat(x).take(i))
                .cycle()
                .skip(1);
            (input
                .iter()
                .zip(cur_pat)
                .skip(i - 1)
                .map(|(a, b)| a * b)
                .sum::<i32>()
                % 10)
                .abs()
        })
        .collect()
}

#[aoc(day16, part1)]
pub fn solve_day16_part1(input: &[i32]) -> String {
    const PATTERN: &[i32] = &[0, 1, 0, -1];
    let mut current = input.to_vec();
    for _ in 0..100 {
        let res = calc_phase(&current, PATTERN);
        current = res;
    }
    current
        .iter()
        .take(8)
        .map(|x| std::char::from_digit(*x as u32, 10).unwrap())
        .collect()
}

#[aoc(day16, part2)]
pub fn solve_day16_part2(input: &[i32]) -> String {
    let offset = input
        .iter()
        .take(7)
        .map(|x| std::char::from_digit(*x as u32, 10).unwrap())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let mut current: Vec<i32> = input
        .iter()
        .cycle()
        .take(10_000 * input.len())
        .skip(offset)
        .map(|&x| x)
        .collect();
    for _ in 0..100 {
        // The fact that we skip over half (important) of the elements means that we can just sum
        // backwards because it's like an upper triangle matrix
        for i in (0..(current.len() - 1)).rev() {
            current[i] += current[i + 1];
            current[i] %= 10;
        }
    }
    current
        .iter()
        .take(8)
        .map(|x| std::char::from_digit(*x as u32, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_phase() {
        assert_eq!(
            calc_phase(&parse_day16("12345678"), &[0, 1, 0, -1]),
            [4, 8, 2, 2, 6, 1, 5, 8]
        );
    }
}
