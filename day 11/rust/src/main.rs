use std::{dbg, fs, println, str::FromStr};

#[derive(Debug)]
enum Operation {
    Add,
    Mult,
}
use Operation::*;

#[derive(Debug)]
enum Token {
    Const(u32),
    Op(Operation),
    Old,
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Token::Old)
        } else if s == "+" {
            Ok(Token::Op(Add))
        } else if s == "*" {
            Ok(Token::Op(Mult))
        } else if let Ok(n) = s.parse() {
            Ok(Token::Const(n))
        } else {
            Err("Unrecognised".to_string())
        }
    }
}

impl Token {
    fn to_num(&self, old: u32) -> Result<u32, ()> {
        match *self {
            Token::Old => Ok(old),
            Token::Const(c) => Ok(c),
            Token::Op(_) => Err(()),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    number: u32,
    inventory: Vec<u32>,
    expr: Vec<Token>,
    div_by: u32,
    pass_throw: u32,
    fail_throw: u32,
    inspected: usize,
}

fn right_of<'a>(line: &'a str, sep: &'a str) -> &'a str {
    let mut split = line.split(sep);
    if let (Some(_), Some(right)) = (split.next(), split.next()) {
        right
    } else {
        ""
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let current_line = lines.next().unwrap();
        let n = current_line.len();
        let number: u32 = current_line[7..n - 1].parse().unwrap();

        let inventory: Vec<u32> = right_of(lines.next().unwrap(), ": ")
            .split(", ")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let expr: Vec<Token> = right_of(lines.next().unwrap(), "= ")
            .split(" ")
            .filter_map(|s| s.parse::<Token>().ok())
            .collect();

        let div_by: u32 = right_of(lines.next().unwrap(), " by ").parse().unwrap();
        let pass_throw: u32 = right_of(lines.next().unwrap(), "monkey ").parse().unwrap();
        let fail_throw: u32 = right_of(lines.next().unwrap(), "monkey ").parse().unwrap();

        Ok(Monkey {
            number,
            inventory,
            expr,
            div_by,
            pass_throw,
            fail_throw,
            inspected: 0,
        })
    }
}

impl Monkey {
    fn op(&self, old: u32) -> u32 {
        let a = self.expr[0].to_num(old).unwrap();
        let b = self.expr[2].to_num(old).unwrap();
        match self.expr[1] {
            Token::Op(Add) => a + b,
            Token::Op(Mult) => a * b,
            _ => panic!("unexpected"),
        }
    }

    fn turn(&mut self) -> Vec<(u32, u32)> {
        let mut throw_list = vec![];
        for &item_wl in self.inventory.iter() {
            // println!("Monkey inspects an item with worry level of {}", item_wl);
            let mut new_wl = self.op(item_wl);
            // println!("Worry level changes to {}", new_wl);
            new_wl /= 3;
            // println!("Monkey gets bored, new worry level is {}", new_wl);
            let test = (new_wl % self.div_by) == 0;
            // println!(
            //     "Current worry level {} divisible by {}",
            //     if test { "is" } else { "is not" },
            //     self.div_by
            // );
            let thrown_to = if test {
                self.pass_throw
            } else {
                self.fail_throw
            };
            // println!(
            //     "Item with worry level {} thrown to monkey {}",
            //     new_wl, thrown_to
            // );
            throw_list.push((new_wl, thrown_to))
        }
        throw_list
    }
}

struct Sim {
    monkeys: Vec<Monkey>,
}

impl Sim {
    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            let throw_list = self.monkeys[i].turn();
            self.monkeys[i].inspected += throw_list.len();
            self.monkeys[i].inventory.clear();
            for (new_wl, thrown_to) in throw_list {
                self.monkeys[thrown_to as usize].inventory.push(new_wl);
            }
        }
    }

    fn part_1(&mut self) {
        for _ in 0..20 {
            self.round();
        }

        let mut activity: Vec<usize> = self.monkeys.iter().map(|m| m.inspected).collect();
        activity.sort();
        activity.reverse();
        dbg!(activity[0] * activity[1]);
    }
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    // let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let mut sim = Sim {
        monkeys: contents
            .split("\n\n")
            .filter_map(|s| s.parse().ok())
            .collect(),
    };

    sim.part_1();
}
