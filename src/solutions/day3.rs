use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse_day3(input: &str) -> Vec<Vec<(char, i32)>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|delta| {
                    let mut delta = delta.chars();
                    (
                        delta.next().unwrap(),
                        delta.collect::<String>().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

fn path_to_points(wire: &[(char, i32)]) -> Vec<(i32, i32)> {
    let mut current = (0, 0);
    wire.iter()
        .map(|(direction, distance)| {
            let start = current;
            match direction {
                'U' => {
                    current.1 += distance;
                    (1..=*distance)
                        .map(|y| (start.0, start.1 + y))
                        .collect::<Vec<_>>()
                }
                'D' => {
                    current.1 -= distance;
                    (1..=*distance)
                        .map(|y| (start.0, start.1 - y))
                        .collect::<Vec<_>>()
                }
                'L' => {
                    current.0 -= distance;
                    (1..=*distance)
                        .map(|x| (start.0 - x, start.1))
                        .collect::<Vec<_>>()
                }
                'R' => {
                    current.0 += distance;
                    (1..=*distance)
                        .map(|x| (start.0 + x, start.1))
                        .collect::<Vec<_>>()
                }
                _ => {
                    panic!("ERROR: {} is not a valid direction", direction);
                }
            }
        })
        .flatten()
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &[Vec<(char, i32)>]) -> i32 {
    let wire1 = path_to_points(&input[1]);
    path_to_points(&input[0])
        .iter()
        .filter(|point0| wire1.iter().any(|point1| *point0 == point1))
        .min_by(|point0, point1| {
            (point0.0.abs() + point0.1.abs()).cmp(&(point1.0.abs() + point1.1.abs()))
        })
        .map(|point| point.0 + point.1)
        .unwrap()
}

fn path_to_points_and_steps(wire: &[(char, i32)]) -> Vec<(i32, i32, i32)> {
    let mut current = (0, 0, 0);
    wire.iter()
        .map(|(direction, distance)| {
            let start = current;
            current.2 += distance;
            match direction {
                'U' => {
                    current.1 += distance;
                    (1..=*distance)
                        .map(|y| (start.0, start.1 + y, start.2 + y))
                        .collect::<Vec<_>>()
                }
                'D' => {
                    current.1 -= distance;
                    (1..=*distance)
                        .map(|y| (start.0, start.1 - y, start.2 + y))
                        .collect::<Vec<_>>()
                }
                'L' => {
                    current.0 -= distance;
                    (1..=*distance)
                        .map(|x| (start.0 - x, start.1, start.2 + x))
                        .collect::<Vec<_>>()
                }
                'R' => {
                    current.0 += distance;
                    (1..=*distance)
                        .map(|x| (start.0 + x, start.1, start.2 + x))
                        .collect::<Vec<_>>()
                }
                _ => {
                    panic!("ERROR: {} is not a valid direction", direction);
                }
            }
        })
        .flatten()
        .collect()
}
#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &[Vec<(char, i32)>]) -> i32 {
    let wire1 = path_to_points_and_steps(&input[1]);
    path_to_points_and_steps(&input[0])
        .iter()
        .filter_map(|point0| {
            if let Some(point1) = wire1
                .iter()
                .find(|point1| point0.0 == point1.0 && point0.1 == point1.1)
            {
                Some((point0, point1))
            } else {
                None
            }
        })
        .min_by(|(point0a, point1a), (point0b, point1b)| {
            (point0a.2 + point1a.2).cmp(&(point0b.2 + point1b.2))
        })
        .map(|(point0, point1)| point0.2 + point1.2)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";
    const EXAMPLE_2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    const EXAMPLE_3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
                             U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn part1_example1() {
        assert_eq!(solve_day3_part1(&parse_day3(EXAMPLE_1)), 6);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(solve_day3_part1(&parse_day3(EXAMPLE_2)), 159);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(solve_day3_part1(&parse_day3(EXAMPLE_3)), 135);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve_day3_part2(&parse_day3(EXAMPLE_1)), 30);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(solve_day3_part2(&parse_day3(EXAMPLE_2)), 610);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(solve_day3_part2(&parse_day3(EXAMPLE_3)), 410);
    }
}
