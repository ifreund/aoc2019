use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
pub struct Moon {
    position: Vec<i64>,
    velocity: Vec<i64>,
}

impl Moon {
    fn new(position: &[i64]) -> Self {
        Self {
            position: position.to_vec(),
            velocity: vec![0, 0, 0],
        }
    }

    fn potential_energy(&self) -> i64 {
        self.position.iter().map(|x| x.abs()).sum()
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.iter().map(|x| x.abs()).sum()
    }

    fn apply_gravity(&mut self, other: &mut Moon) {
        for ((self_p, self_v), (other_p, other_v)) in self
            .position
            .iter_mut()
            .zip(self.velocity.iter_mut())
            .zip(other.position.iter_mut().zip(other.velocity.iter_mut()))
        {
            match (*self_p, *other_p) {
                (a, b) if a > b => {
                    *self_v -= 1;
                    *other_v += 1;
                }
                (a, b) if a < b => {
                    *self_v += 1;
                    *other_v -= 1;
                }
                _ => (),
            }
        }
    }

    fn apply_velocity(&mut self) {
        for (p, v) in self.position.iter_mut().zip(self.velocity.iter()) {
            *p += v;
        }
    }
}

#[aoc_generator(day12)]
pub fn parse_day12(input: &str) -> Vec<Moon> {
    let input = input
        .chars()
        .filter(|c| match c {
            '<' | '>' | 'x' | 'y' | 'z' | '=' | ' ' => false,
            _ => true,
        })
        .collect::<String>();
    input
        .lines()
        .map(|line| {
            Moon::new(
                &*line
                    .split(',')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in (i + 1)..moons.len() {
            let mut moon1 = moons[i].clone();
            moon1.apply_gravity(&mut moons[j]);
            moons[i] = moon1;
        }
    }
    for moon in moons.iter_mut() {
        moon.apply_velocity();
    }
}

#[aoc(day12, part1)]
pub fn solve_day12_part1(input: &[Moon]) -> i64 {
    let mut moons = input.to_vec();
    for _ in 0..1000 {
        step(&mut moons);
    }
    moons
        .iter()
        .map(|moon| moon.potential_energy() * moon.kinetic_energy())
        .sum()
}

fn equal_on_axis(orig: &[Moon], current: &[Moon], axis: usize) -> bool {
    for (moon1, moon2) in orig.iter().zip(current.iter()) {
        if moon1.position[axis] != moon2.position[axis]
            || moon1.velocity[axis] != moon2.velocity[axis]
        {
            return false;
        }
    }
    true
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

#[aoc(day12, part2)]
pub fn solve_day12_part2(input: &[Moon]) -> i64 {
    let mut moons = input.to_vec();
    let mut periods = [0; 3];
    let mut count = 0;
    loop {
        step(&mut moons);
        count += 1;
        for (i, period) in periods.iter_mut().enumerate() {
            if *period == 0 && equal_on_axis(input, &moons, i) {
                *period = count;
            }
        }
        if periods.iter().all(|x| *x != 0) {
            break;
        }
    }
    lcm(periods[0], lcm(periods[1], periods[2]))
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE_1: &str = "<x=-1, y=0, z=2>\n\
                             <x=2, y=-10, z=-7>\n\
                             <x=4, y=-8, z=8>\n\
                             <x=3, y=5, z=-1>";

    const EXAMPLE_2: &str = "<x=-8, y=-10, z=0>\n\
                             <x=5, y=5, z=10>\n\
                             <x=2, y=-7, z=3>\n\
                             <x=9, y=-8, z=-3>";

    #[test]
    fn example_1() {
        assert_eq!(solve_day12_part2(&parse_day12(EXAMPLE_1)), 2772);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_day12_part2(&parse_day12(EXAMPLE_2)), 4686774924);
    }
}
