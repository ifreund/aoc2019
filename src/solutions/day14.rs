use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;
use std::collections::HashMap;

#[aoc_generator(day14)]
pub fn parse_day14(input: &str) -> HashMap<String, (i64, Vec<(i64, String)>)> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(" => ");
            let components = line
                .next()
                .unwrap()
                .split(", ")
                .map(|component| {
                    let mut component = component.split(' ');
                    (
                        component.next().unwrap().parse::<i64>().unwrap(),
                        component.next().unwrap().to_owned(),
                    )
                })
                .collect::<Vec<_>>();
            let mut product = line.next().unwrap().split(' ');
            let product_count = product.next().unwrap().parse::<i64>().unwrap();
            let product = product.next().unwrap();
            (product.to_owned(), (product_count, components))
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_day14_part1(input: &HashMap<String, (i64, Vec<(i64, String)>)>) -> i64 {
    let mut reqs = HashMap::new();
    reqs.insert("FUEL", 1);
    while let Some((req, req_count)) = reqs
        .iter()
        .find(|(req, req_count)| **req != "ORE" && **req_count > 0)
    {
        if let Some((product_count, components)) = input.get(*req) {
            let factor = max(req_count / product_count, 1);
            // This is to make the borrow checker a litle happier
            let req = *req;
            reqs.entry(&req)
                .and_modify(|x| *x -= product_count * factor);
            for (comp_count, comp) in components {
                reqs.entry(comp)
                    .and_modify(|x| *x += comp_count * factor)
                    .or_insert(*comp_count * factor);
            }
        } else {
            panic!("ERROR: invalid req: {}", req);
        }
    }

    *reqs.get("ORE").unwrap()
}

#[aoc(day14, part2)]
pub fn solve_day14_part2(input: &HashMap<String, (i64, Vec<(i64, String)>)>) -> i64 {
    // A rough estimate based on part 1
    let mut fuel = 1_000_000_000_000 / 843220 * 3 / 2;
    let mut reqs = HashMap::new();
    reqs.insert("ORE", -1_000_000_000_000);
    reqs.insert("FUEL", fuel);
    while let Some(ore_req) = reqs.get("ORE") {
        if *ore_req > 0 {
            break;
        }
        reqs.entry("FUEL").and_modify(|x| *x += 1);
        fuel += 1;
        while let Some((req, req_count)) = reqs
            .iter()
            .find(|(req, req_count)| **req != "ORE" && **req_count > 0)
        {
            if let Some((product_count, components)) = input.get(*req) {
                let factor = max(req_count / product_count, 1);
                // This is to make the borrow checker a litle happier
                let req = *req;
                reqs.entry(&req)
                    .and_modify(|x| *x -= product_count * factor);
                for (comp_count, comp) in components {
                    reqs.entry(comp)
                        .and_modify(|x| *x += comp_count * factor)
                        .or_insert(*comp_count * factor);
                }
            } else {
                panic!("ERROR: invalid req: {}", req);
            }
        }
    }
    fuel - 1
}
