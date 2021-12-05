use std::io;
use std::io::BufRead;

use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Forward,
    Back, // Problem statement doesn't seem to allow.
    Up,
    Down,
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "forward" => Ok(Direction::Forward),
            "back" => Ok(Direction::Back),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(format!("Could not convert '{}' to Command", s)),
        }
    }
}

#[derive(Debug)]
struct Position {
    pub depth: i32,
    pub horizontal: i32,
}

impl Position {
    fn forward(&mut self, val: i32) {
        self.horizontal += val;
    }
    fn back(&mut self, val: i32) {
        self.horizontal -= val;
    }
    fn up(&mut self, val: i32) {
        self.depth -= val;
    }
    fn down(&mut self, val: i32) {
        self.depth += val;
    }

    fn interpret(&mut self, command: Command) {
        match command.direction {
            Direction::Forward => self.forward(command.amount),
            Direction::Back => self.back(command.amount),
            Direction::Up => self.up(command.amount),
            Direction::Down => self.down(command.amount),
        }
    }
}

#[derive(Debug)]
struct Command {
    pub direction: Direction,
    pub amount: i32,
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split_whitespace();
        let direction: Direction = iter
            .next()
            .expect("Failed to split direction")
            .parse()
            .expect("Failed to parse direction");
        let amount: i32 = iter
            .next()
            .expect("Failed to split amount")
            .parse()
            .expect("Failed to parse amount");

        Ok(Command { direction, amount })
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let commands = lines.map(|line| line.expect("Failed to read line").parse::<Command>().expect("Failed to parse line as command"));

    let mut sub = Position {
        depth: 0,
        horizontal: 0,
    };

    for command in commands {
        sub.interpret(command);
        println!("New position: ({}, {})", sub.depth, sub.horizontal);
    }

    println!("final product: {}", sub.depth * sub.horizontal);
}
