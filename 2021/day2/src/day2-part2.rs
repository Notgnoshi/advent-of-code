use std::io;
use std::io::BufRead;

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
struct Submarine {
    pub depth: i32,
    pub aim: i32,
    pub horizontal: i32,
}

impl Submarine {
    fn new() -> Self {
        Self {
            depth: 0,
            aim: 0,
            horizontal: 0,
        }
    }
    fn forward(&mut self, val: i32) {
        self.horizontal += val;
        self.depth += self.aim * val;
    }
    fn up(&mut self, val: i32) {
        self.aim -= val;
    }
    fn down(&mut self, val: i32) {
        self.aim += val;
    }

    fn interpret(&mut self, command: Command) {
        match command.direction {
            Direction::Forward => self.forward(command.amount),
            Direction::Back => panic!("You can't go backwards!"),
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
    let commands = lines.map(|line| {
        line.expect("Failed to read line")
            .parse::<Command>()
            .expect("Failed to parse line as command")
    });

    let mut sub = Submarine::new();

    for command in commands {
        sub.interpret(command);
        println!(
            "New position: Submarine<depth={}, aim={}, horizontal={}>",
            sub.depth, sub.aim, sub.horizontal
        );
    }

    println!("final product: {}", sub.depth * sub.horizontal);
}
