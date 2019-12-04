use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn parse_day1(input: &str) -> (u32, u32) {
    let mut it = input.split('-').map(|x| x.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap())
}

#[aoc(day4, part1)]
pub fn solve_day4_part1(input: &(u32, u32)) -> usize {
    (input.0..=input.1)
        .map(|number| {
            number
                .to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|digits| {
            digits.iter().is_sorted() && digits.windows(2).any(|pair| pair[0] == pair[1])
        })
        .count()
}

fn exactly_2_consecutive(digits: &[u32]) -> bool {
    let mut consecutive_count = 1;
    let mut previous = 10;
    for digit in digits {
        if *digit == previous {
            consecutive_count += 1;
        } else if consecutive_count == 2 {
            return true;
        } else {
            consecutive_count = 1;
        }
        previous = *digit;
    }
    consecutive_count == 2
}

#[aoc(day4, part2)]
pub fn solve_day4_part2(input: &(u32, u32)) -> usize {
    (input.0..=input.1)
        .map(|number| {
            number
                .to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|digits| digits.iter().is_sorted() && exactly_2_consecutive(&digits))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exactly_2_consecutive_test1() {
        assert_eq!(exactly_2_consecutive(&[1, 1, 2, 2, 3, 3]), true);
    }

    #[test]
    fn exactly_2_consecutive_test2() {
        assert_eq!(exactly_2_consecutive(&[1, 2, 3, 4, 4, 4]), false);
    }
    #[test]
    fn exactly_2_consecutive_test3() {
        assert_eq!(exactly_2_consecutive(&[1, 1, 1, 1, 2, 2]), true);
    }
}
