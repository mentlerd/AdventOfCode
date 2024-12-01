use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::ops::AddAssign;

fn main() {
    let input = parse_input();

    assert_eq!(part1(input.clone()), 936_063);
    assert_eq!(part2(&input), 23_150_395);
}

#[derive(Clone)]
struct Input {
    list0: Vec<u64>,
    list1: Vec<u64>,
}

fn parse_input() -> Input {
    let file = File::open("data/input.txt").expect("input.txt missing");

    let reader = BufReader::new(file);

    let mut list0 = Vec::new();
    let mut list1 = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("unable to read line");

        let numbers: Vec<u64> = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if numbers.len() != 2 {
            panic!("Malformed line #{i}: '{line}'");
        }

        list0.push(numbers[0]);
        list1.push(numbers[1]);
    }

    Input { list0, list1 }
}

fn part1(mut input: Input) -> u64 {
    input.list0.sort();
    input.list1.sort();

    zip(input.list0, input.list1)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part2(input: &Input) -> u64 {
    let histogram = input
        .list1
        .iter()
        .fold(HashMap::new(), |mut histogram, number| {
            histogram.entry(*number).or_insert(0).add_assign(1);
            histogram
        });

    input
        .list0
        .iter()
        .map(|number| number * histogram.get(number).unwrap_or(&0))
        .sum()
}
