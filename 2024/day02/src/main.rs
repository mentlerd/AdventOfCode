use std::cmp::{Ordering};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_safe(numbers: &[u64]) -> bool {
    assert!(numbers.len() > 1);

    let ordering = numbers[0].cmp(&numbers[1]);
    if ordering == Ordering::Equal {
        return false;
    }

    numbers.windows(2).all(|w| w[0].cmp(&w[1]) == ordering && w[0].abs_diff(w[1]) <= 3)
}

fn part1() -> u64 {
    let file = File::open("data/input.txt").expect("input.txt missing");

    let reader = BufReader::new(file);

    let mut safe_records = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("unable to read line");

        let numbers: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if numbers.len() < 2 {
            panic!("Malformed line #{i}: '{line}'");
        }

        if is_safe(numbers.as_slice()) {
            safe_records += 1;
        }
    }

    safe_records
}

struct Stream {
    ordering: Ordering,
    last: Option<u64>,
    faults: u64
}

impl Stream {
    fn accept(&mut self, number: u64) {
        if let Some(last) = self.last {
            if last.abs_diff(number) > 3 {
                self.faults += 1;
                return;
            } else if last.cmp(&number) != self.ordering {
                self.faults += 1;
                return;
            }
        }

        self.last = Some(number)
    }

    fn valid(&self) -> bool {
        self.faults < 2
    }
}

fn part2() -> u64 {
    let file = File::open("data/input.txt").expect("input.txt missing");

    let reader = BufReader::new(file);

    let mut safe_records = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("unable to read line");

        let numbers: Vec<u64> = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if numbers.len() < 2 {
            panic!("Malformed line #{i}: '{line}'");
        }

        // Self-imposed limitation: O(N)
        let mut streams = [
            Stream { ordering: Ordering::Less, last: None, faults: 1, },
            Stream { ordering: Ordering::Less, last: Some(numbers[0]), faults: 0, },
            Stream { ordering: Ordering::Greater, last: None, faults: 1, },
            Stream { ordering: Ordering::Greater, last: Some(numbers[0]), faults: 0, },
        ];

        let safe = numbers.iter().skip(1).all(|number| {
            for stream in streams.iter_mut() {
                stream.accept(*number);
            }

            streams.iter().any(|s| s.valid())
        });

        if safe {
            safe_records += 1;
        }
    }

    safe_records
}

fn main() {
    assert_eq!(part1(), 483);
    assert_eq!(part2(), 528);
}
