use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, PartialEq)]
enum Token {
    Garbage,
    Mul,
    LParen,
    Number(u64),
    Comma,
    RParen,
    EOF,

    // Part2
    Do,
    Dont,
}

struct Lexer {
    next: Option<u8>,
    iterator: Box<dyn Iterator<Item=u8>>,
}

impl Lexer {
    fn new(reader: BufReader<File>) -> Lexer {
        let mut iterator = reader.bytes().map(|l| l.unwrap());

        Self {
            next: iterator.next(),
            iterator: Box::new(iterator),
        }
    }

    fn peek_any(&mut self) -> Option<u8> {
        self.next
    }
    fn consume_any(&mut self) -> Option<u8> {
        if self.next.is_none() {
            return None
        }

        let consumed = self.next;
        self.next = self.iterator.next();
        consumed
    }

    fn peek(&mut self, ascii: u8) -> bool {
        self.peek_any() == Some(ascii)
    }
    fn consume(&mut self, ascii: u8) -> bool {
        if !self.peek(ascii) {
            return false
        }
        self.consume_any();
        true
    }

    fn peek_digit(&mut self) -> Option<u64> {
        let Some(next) = self.peek_any() else {
            return None
        };

        let (digit, _) = next.overflowing_sub(b'0');
        if digit > 9 {
            return None
        }
        Some(digit as u64)
    }
    fn consume_digit(&mut self) -> Option<u64> {
        let Some(digit) = self.peek_digit() else {
            return None
        };
        self.consume_any();
        Some(digit)
    }

    fn next(&mut self) -> Token {
        if self.consume(b'm') {
            if self.consume(b'u') {
                if self.consume(b'l') {
                    return Token::Mul
                }
            }

            return Token::Garbage
        }

        if self.consume(b'd') {
            if self.consume(b'o') {
                if self.consume(b'n') {
                    if self.consume(b'\'') {
                        if self.consume(b't') {
                            return Token::Dont
                        }
                    }

                    return Token::Garbage;
                }
                return Token::Do;
            }
            return Token::Garbage
        }

        if let Some(digit) = self.consume_digit() {
            let mut value = digit;

            for _ in 0..3 {
                let Some(digit) = self.consume_digit() else {
                    return Token::Number(value)
                };

                value *= 10;
                value += digit
            }
            return Token::Garbage
        }

        let Some(char) = self.consume_any() else {
            return Token::EOF
        };

        match char {
            b'(' => Token::LParen,
            b',' => Token::Comma,
            b')' => Token::RParen,

            _ => Token::Garbage
        }
    }
}

fn part1() -> u64 {
    let file = File::open("data/input.txt").expect("input.txt missing");
    let reader = BufReader::new(file);

    let mut lexer = Lexer::new(reader);
    let mut result = 0;

    while lexer.next.is_some() {
        if lexer.next() != Token::Mul {
            continue
        }
        if lexer.next() != Token::LParen {
            continue
        }
        let Token::Number(lhs) = lexer.next() else {
            continue
        };
        if lexer.next() != Token::Comma {
            continue
        }
        let Token::Number(rhs) = lexer.next() else {
            continue
        };
        if lexer.next() != Token::RParen {
            continue
        }

        result += lhs * rhs;
    }

    result
}

fn part2() -> u64 {
    let file = File::open("data/input.txt").expect("input.txt missing");
    let reader = BufReader::new(file);

    let mut lexer = Lexer::new(reader);

    let mut enable = true;
    let mut result = 0;

    while lexer.next.is_some() {
        match lexer.next() {
            Token::Mul => {
                if lexer.next() != Token::LParen {
                    continue
                }
                let Token::Number(lhs) = lexer.next() else {
                    continue
                };
                if lexer.next() != Token::Comma {
                    continue
                }
                let Token::Number(rhs) = lexer.next() else {
                    continue
                };
                if lexer.next() != Token::RParen {
                    continue
                }

                if enable {
                    result += lhs * rhs;
                }
            }
            Token::Do => enable = true,
            Token::Dont => enable = false,
            _ => {}
        }
    }

    result
}

fn main() {
    assert_eq!(part1(), 174_103_751);
    assert_eq!(part2(), 100_411_201);
}
