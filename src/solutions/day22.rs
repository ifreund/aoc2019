use aoc_runner_derive::{aoc, aoc_generator};

pub enum Tech {
    Stack,
    Cut(i128),
    Inc(i128),
}

#[aoc_generator(day22)]
pub fn parse_day22(input: &str) -> Vec<Tech> {
    input
        .lines()
        .map(|line| match line {
            "deal into new stack" => Tech::Stack,
            line if line.starts_with("cut ") => {
                Tech::Cut(line.split_whitespace().last().unwrap().parse().unwrap())
            }
            line if line.starts_with("deal with increment ") => {
                Tech::Inc(line.split_whitespace().last().unwrap().parse().unwrap())
            }
            _ => panic!("ERROR: invalid input \"{}\"", line),
        })
        .collect()
}

#[aoc(day22, part1)]
pub fn solve_day22_part1(input: &[Tech]) -> i128 {
    const DECK_SIZE: i128 = 10007;
    let mut pos = 2019;
    for tech in input {
        match tech {
            Tech::Stack => pos = DECK_SIZE - 1 - pos,
            Tech::Cut(x) => pos = (pos - x) % DECK_SIZE,
            Tech::Inc(x) => pos = (pos * x) % DECK_SIZE,
        }
    }
    pos
}

fn extended_euclid(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_euclid(b.rem_euclid(a), a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(a: i128, modulus: i128) -> i128 {
    let (g, x, _) = extended_euclid(a, modulus);
    // Would indicate nonexistence of a modinv for the given values
    assert!(g == 1);
    x.rem_euclid(modulus)
}

fn mod_pow(base: i128, exponent: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        0
    } else {
        let mut res = 1;
        let mut base = base % modulus;
        let mut exponent = exponent;
        while exponent > 0 {
            if exponent % 2 == 1 {
                res = (res * base) % modulus;
            }
            exponent >>= 1;
            base = base * base % modulus;
        }
        res
    }
}

#[aoc(day22, part2)]
pub fn solve_day22_part2(input: &[Tech]) -> i128 {
    const DECK_SIZE: i128 = 119315717514047;
    const NUM_SHUFFLES: i128 = 101741582076661;
    let mut a = 1;
    let mut b = 0;
    for tech in input {
        match tech {
            Tech::Stack => {
                a = -a;
                b = -b;
                b += DECK_SIZE - 1
            }
            Tech::Cut(x) => {
                b += DECK_SIZE - x;
            }
            Tech::Inc(x) => {
                a *= x;
                b *= x;
            }
        }
        a = a.rem_euclid(DECK_SIZE);
        b = b.rem_euclid(DECK_SIZE);
    }

    // For n = NUM_SHUFFLES iterations full_a = a^n, full_b = (a^n - 1) / (a - 1) * b
    let an = mod_pow(a, NUM_SHUFFLES, DECK_SIZE);
    let b = ((an - 1) * mod_inv(a - 1, DECK_SIZE)).rem_euclid(DECK_SIZE) * b;
    let b = b.rem_euclid(DECK_SIZE);
    let a = an;

    // the final position y of a card at initial position x after a full shuffle
    // is given by y = ax + b mod DECK_SIZE
    // however, we want the initial position of a card at final positon y,
    // so we use x = (y - b) / a mod DECK_SIZE
    // however we need to use modular multiplicative inverse instead of /
    ((2020 - b) * mod_inv(a, DECK_SIZE)).rem_euclid(DECK_SIZE)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mod_inv_test() {
        assert_eq!(mod_inv(3, 26), 9);
    }

    #[test]
    fn mod_pow_test() {
        assert_eq!(mod_pow(4, 13, 497), 445);
    }
}
