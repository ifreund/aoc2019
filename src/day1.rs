#[aoc_generator(day1)]
pub fn parse_day1(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_day1_part1(input: &[i32]) -> i32 {
    input.iter().map(|x| x / 3 - 2).sum()
}

fn calc_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + calc_fuel(fuel)
    }
}

#[aoc(day1, part2)]
pub fn solve_day1_part2(input: &[i32]) -> i32 {
    input.iter().map(|&x| calc_fuel(x)).sum()
}
