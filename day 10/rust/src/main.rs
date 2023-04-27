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
        return Err("parse failure, invalid instruction name?".to_string());
    }
}

struct Emulator {
    register: i32,
    cycle: usize,
    strength_sum: i32,
    debug: bool
}

impl Emulator {
    
    fn new(debug: bool) -> Self {
        Emulator { register: 1, cycle: 1, strength_sum:0, debug }
    }

    fn crt(&self) {
        let screen_pos: usize = (self.cycle-1)%40;

        let c = if (self.register-1..=self.register+1).contains(&(screen_pos as i32)) 
                       { '#' } else { '.' };

        if self.debug {
            println!("CRT prints {c} at {}", screen_pos);
        } else {
            print!("{c}");
            if self.cycle%40 == 0 { println!(); }
        }
    }

    fn cycle(&mut self) {

        // part 1
        if self.cycle>=20 {
            let offset_cycle = self.cycle - 20;
            if offset_cycle%40==0 {
                // dbg!((self.cycle as i32) * self.register);
                self.strength_sum += (self.cycle as i32) * self.register;
            }
        }

        // part 2
        self.crt();

        self.cycle += 1;
    }

    fn add(&mut self, v: i32) {
        if self.debug { println!("begin executing add {v}"); }
        self.cycle();
        self.cycle();
        self.register += v;
        if self.debug { println!("finish executing add (Register is now {})", self.register); }
    }

    fn noop(&mut self) {
        if self.debug { println!("begin executing noop"); }
        self.cycle();
    }

    fn exec(&mut self, ins: Instruction) {
        match ins {
            Addx(v) => self.add(v),
            Noop         => self.noop()
        }
    }

    fn run(&mut self, prog: Vec<Instruction>) {
        for ins in prog {
            if self.debug { print!("Start cycle {}: ", self.cycle); }
            self.exec(ins);
            if self.debug { println!(); }
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
        let prog: Vec<Instruction> = tcontents.lines().map(|l| l.parse().unwrap()).collect();
        let mut emu = Emulator::new(false);
        emu.run(prog);
        assert_eq!(emu.strength_sum, 13140);
    }
}

fn main() {
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let prog: Vec<Instruction> = contents.lines().map(|l| l.parse().unwrap()).collect();

    let mut emu = Emulator::new(false);

    emu.run(prog);

}
