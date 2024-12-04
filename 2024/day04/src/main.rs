use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    INITIAL,
    X,
    XM,
    XMA,
    S,
    SA,
    SAM,
}

fn advance(state: State, char: char) -> (State, bool) {
    match (state, char) {
        (State::X, 'M') => (State::XM, false),
        (State::XM, 'A') => (State::XMA, false),
        (State::XMA, 'S') => (State::S, true),

        (State::S, 'A') => (State::SA, false),
        (State::SA, 'M') => (State::SAM, false),
        (State::SAM, 'X') => (State::X, true),

        (_, 'X') => (State::X, false),
        (_, 'S') => (State::S, false),

        (_, _) => (State::INITIAL, false),
    }
}

#[derive(Debug, Copy, Clone)]
struct Detector {
    state: State,
}

impl Detector {
    fn new() -> Detector {
        Self { state: State::INITIAL }
    }

    fn reset(&mut self) {
        self.state = State::INITIAL;
    }

    fn advance(&mut self, char: char) -> bool {
        let (new_state, detected) = advance(self.state, char);

        self.state = new_state;
        detected
    }

    fn advance_inc(&mut self, char: char) -> u64 {
        if self.advance(char) { 1 } else { 0 }
    }
}

fn part1() -> u64 {
    let file = File::open("data/input.txt").expect("input.txt missing");
    let reader = BufReader::new(file);

    let mut count = 0;

    // Horizontal detection of XMAS can be done by a single detector
    let mut horizontal_detector = Detector::new();

    // We keep as many vertical detectors as there are columns
    let mut vertical_detectors = Vec::new();

    // Diagonal detectors are vertical detectors which we strategically move
    // on the horizontal axis when advancing a line, making the input text
    // appear to them as-if it was vertical:
    //
    // Left diagonal | Right diagonal
    // X...     X... | ...X  ...X
    // .M..    .M..  | ..M.   ..M.
    // ..A.   ..A.   | .A..    .A..
    // ...S  ...S.   | S...     S..
    let mut left_diagonal_detectors = VecDeque::new();
    let mut right_diagonal_detectors = VecDeque::new();

    for line in reader.lines() {
        let line = line.unwrap();

        // Assuming input data lines are always the same width
        if vertical_detectors.is_empty() {
            let w = line.len();

            vertical_detectors.resize(w, Detector::new());
            left_diagonal_detectors.resize(w, Detector::new());
            right_diagonal_detectors.resize(w, Detector::new());
        }

        horizontal_detector.reset();

        left_diagonal_detectors.rotate_right(1);
        left_diagonal_detectors.front_mut().unwrap().reset();

        right_diagonal_detectors.rotate_left(1);
        right_diagonal_detectors.back_mut().unwrap().reset();

        for (i, char) in line.chars().enumerate() {
            count += horizontal_detector.advance_inc(char);
            count += vertical_detectors[i].advance_inc(char);
            count += left_diagonal_detectors[i].advance_inc(char);
            count += right_diagonal_detectors[i].advance_inc(char);
        }
    }

    count
}

// Feeling a little lazy today, and my solution easily generalizes for this case too
// .. copy paste it is! :)

#[derive(Debug, PartialEq, Copy, Clone)]
enum State2 {
    INITIAL,
    M,
    MA,
    S,
    SA,
}

fn advance2(state: State2, char: char) -> (State2, bool) {
    match (state, char) {
        (State2::M, 'A') => (State2::MA, false),
        (State2::MA, 'S') => (State2::S, true),

        (State2::S, 'A') => (State2::SA, false),
        (State2::SA, 'M') => (State2::M, true),

        (_, 'M') => (State2::M, false),
        (_, 'S') => (State2::S, false),

        (_, _) => (State2::INITIAL, false),
    }
}

#[derive(Debug, Copy, Clone)]
struct Detector2 {
    state: State2,
}

impl Detector2 {
    fn new() -> Detector2 {
        Self { state: State2::INITIAL }
    }

    fn reset(&mut self) {
        self.state = State2::INITIAL;
    }

    fn advance(&mut self, char: char) -> bool {
        let (new_state, detected) = crate::advance2(self.state, char);

        self.state = new_state;
        detected
    }
}

fn part2() -> u64 {
    let file = File::open("data/input.txt").expect("input.txt missing");
    let reader = BufReader::new(file);

    let mut count = 0;

    let mut left_diagonal_detectors = VecDeque::new();
    let mut right_diagonal_detectors = VecDeque::new();

    // Staggering the right diagonal detector's output is a neat trick so
    // we can detect the cross pattern. If we found a left diagonal, and
    // we have found a right diagonal two columns ago, they form an X
    let mut right_detected_1_ago = false;
    let mut right_detected_2_ago = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if left_diagonal_detectors.is_empty() {
            let w = line.len();

            left_diagonal_detectors.resize(w, Detector2::new());
            right_diagonal_detectors.resize(w, Detector2::new());
        }

        left_diagonal_detectors.rotate_right(1);
        left_diagonal_detectors.front_mut().unwrap().reset();

        right_diagonal_detectors.rotate_left(1);
        right_diagonal_detectors.back_mut().unwrap().reset();

        for (i, char) in line.chars().enumerate() {
            if left_diagonal_detectors[i].advance(char) && right_detected_2_ago {
                count += 1;
            }

            right_detected_2_ago = right_detected_1_ago;
            right_detected_1_ago = right_diagonal_detectors[i].advance(char);
        }
    }

    count
}

fn main() {
    assert_eq!(part1(), 2562);
    assert_eq!(part2(), 1902);
}
