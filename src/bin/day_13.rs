use regex::Regex;
use std::fs;

const BUTTON_A_PATTERN: &str = r"Button A: X\+(\d+), Y\+(\d+)";
const BUTTON_B_PATTERN: &str = r"Button B: X\+(\d+), Y\+(\d+)";
const PRIZE_PATTERN: &str = r"Prize: X=(\d+), Y=(\d+)";

const MAX_BUTTON_PRESSED: i32 = 100;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Position,
    button_b: Position,
    prize: Position,
    token_price: i32,
}

impl ClawMachine {
    fn is_solved(&self) -> bool {
        self.token_price > 0
    }

    fn solve(&mut self) {
        for i in 0..MAX_BUTTON_PRESSED + 1 {
            for j in 0..MAX_BUTTON_PRESSED + 1 {
                if self.button_a.x * i + self.button_b.x * j == self.prize.x
                    && self.button_a.y * i + self.button_b.y * j == self.prize.y
                {
                    self.token_price = i * 3 + j;
                    return;
                }
            }
        }
    }
}

fn parse_position(input: &str, pattern: &str) -> Option<Position> {
    let regex = Regex::new(pattern).unwrap();
    regex.captures(input).and_then(|captures| {
        let x = captures.get(1)?.as_str().parse::<i32>().ok()?;
        let y = captures.get(2)?.as_str().parse::<i32>().ok()?;
        Some(Position { x, y })
    })
}

fn load_input() -> Vec<ClawMachine> {
    let binding = fs::read_to_string("./resources/day_13.txt").unwrap();
    let splits = binding.split("\n\n").collect::<Vec<&str>>();

    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    for split in splits {
        let button_a = parse_position(split, BUTTON_A_PATTERN).unwrap();
        let button_b = parse_position(split, BUTTON_B_PATTERN).unwrap();
        let prize = parse_position(split, PRIZE_PATTERN).unwrap();
        claw_machines.push(ClawMachine {
            button_a,
            button_b,
            prize,
            token_price: -1,
        });
    }
    claw_machines
}

fn main() {
    let mut claw_machines = load_input();
    let mut total_token_price = 0;
    for claw_machine in claw_machines.iter_mut() {
        claw_machine.solve();
        if claw_machine.is_solved() {
            total_token_price += claw_machine.token_price;
        }
    }
    println!("Total token price: {}", total_token_price);
}
