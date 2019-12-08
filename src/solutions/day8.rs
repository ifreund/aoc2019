use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
pub fn parse_day8(input: &str) -> Vec<Vec<u32>> {
    input
        .as_bytes()
        .chunks(25 * 6)
        .map(|layer| {
            std::str::from_utf8(layer)
                .unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_day8_part1(raw_image: &[Vec<u32>]) -> usize {
    let count_digits = |layer: &[u32], digit| layer.iter().filter(|x| **x == digit).count();
    raw_image
        .iter()
        .min_by_key(|layer| count_digits(layer, 0))
        .map(|layer| count_digits(layer, 1) * count_digits(layer, 2))
        .unwrap()
}

#[aoc(day8, part2)]
pub fn solve_day8_part2(raw_image: &[Vec<u32>]) -> String {
    raw_image
        .iter()
        .rev()
        .fold(vec![2; 25 * 6], |mut acc, layer| {
            for (i, pixel) in layer.iter().enumerate() {
                if *pixel != 2 {
                    acc[i] = *pixel
                }
            }
            acc
        })
        .chunks(25)
        .map(|row| {
            row.iter()
                .map(|x| match x {
                    0 => '█',
                    1 => ' ',
                    _ => unreachable!(),
                })
                .collect::<String>()
        })
        .fold("\n".to_owned(), |mut acc, row| {
            acc.push_str(&row);
            acc.push('\n');
            acc
        })
}
