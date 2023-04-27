use std::{str::FromStr, fs};

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop
}
use Instruction::*;

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        if let Some(ins_str) = s.next() {
            if ins_str=="addx" {
                if let Some(val_str) = s.next() {
                    let v: i32 = val_str.parse().expect("expected integer after whitespace");
                    return Ok(Addx(v));
                }
            } else if ins_str=="noop" {
                return Ok(Noop);
            }
        }
        return Err("parse failure".to_string());
    }
}

struct Emulator {
    register: i32,
    cycle: usize,
    strength_sum: i32
}

impl Emulator {
    
    fn new() -> Self {
        Emulator { register: 1, cycle: 0, strength_sum:0 }
    }

    fn cycle(&mut self) {
        self.cycle += 1;

        if self.cycle>=20 {
            let offset_cycle = self.cycle - 20;
            if offset_cycle%40==0 {
                // dbg!((self.cycle as i32) * self.register);
                self.strength_sum += (self.cycle as i32) * self.register;
            }
        }

    }

    fn add(&mut self, v: i32) {
        self.cycle();
        self.cycle();
        self.register += v;
    }

    fn exec(&mut self, ins: Instruction) {
        match ins {
            Addx(v) => self.add(v),
            Noop         => self.cycle()
        }
    }

    fn run(&mut self, inss: Vec<Instruction>) {
        for ins in inss {
            self.exec(ins);
        }
    }

}

static INPUT_PATH : &str = "../input";
static TEST_INPUT_PATH : &str = "../test_input";

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1() {
        let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
        let instructions: Vec<Instruction> = tcontents.lines().map(|l| l.parse().unwrap()).collect();
        let mut emu = Emulator::new();
        emu.run(instructions);
        assert_eq!(emu.strength_sum, 13140);
    }
}

fn main() {
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let instructions: Vec<Instruction> = contents.lines().map(|l| l.parse().unwrap()).collect();

    let mut emu = Emulator::new();

    emu.run(instructions);

    dbg!(emu.strength_sum);
}
