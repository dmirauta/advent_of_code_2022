use std::{str::FromStr, fs, collections::{HashSet}};

type Pos = (usize, usize);

#[derive(Debug)]
struct RopeSim {
    head: Pos,
    tail: Pos,
    size: Pos,
    tail_trace: HashSet<Pos>
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}
use Direction::*;

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Left),
            'R' => Ok(Right),
            'D' => Ok(Down),
            'U' => Ok(Up),
             _  => Err("Not a direction".to_string())
        }
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    amount: usize
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let dir: Direction = it.next().unwrap().try_into().unwrap();
        it.next();
        let amount: usize = it.next().unwrap().to_string().parse().unwrap();
        Ok( Instruction { dir, amount } )
    }
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        let c = match self.dir {
            Left => 'L',
            Right => 'R',
            Down => 'D',
            Up => 'U'
        };
        format!("{} {}", c, self.amount)
    }
}

fn inc_pos((i,j):Pos, dir: Direction, bounds: Pos) -> Pos {
    match dir {
        Left  => (i.checked_sub(1).unwrap_or(i), j),
        Right => (i+1, j),
        Up    => (i, j+1),
        Down  => (i, j.checked_sub(1).unwrap_or(j))
    }
}

impl RopeSim {
    fn new(head:Pos, tail:Pos) -> Self {
        let tail_trace: HashSet<Pos> = HashSet::new();
        let size: Pos = (1,1);
        RopeSim { head, tail, size, tail_trace }
    }

    fn calc_size(&mut self) {
        let (i_max,_) = self.tail_trace.iter().max_by_key(|(i,_)| i).unwrap();
        let (_, j_max) = self.tail_trace.iter().max_by_key(|(_,j)| j).unwrap();
        self.size = (*i_max, *j_max);
    }

    fn draw_state(&self) {
        for j in (0..self.size.1).rev() {
            for i in 0..self.size.0 {
                let c = if (i,j)==self.head { "H" } 
                         else if (i,j)==self.tail { "T" } 
                         else { "." };
                print!("{c}");
            }
            println!();
        }
    }

    fn draw_tail_trace(&self) {
        for j in (0..self.size.1).rev() {
            for i in 0..self.size.0 {
                let c = if self.tail_trace.contains(&(i,j)) { "#" }
                         else { "." };
                print!("{c}");
            }
            println!();
        }
    }

    fn step(&mut self, dir: Direction) {
        self.head = inc_pos(self.head, dir, self.size);
        let (ni, nj) = self.head;

        let delta_i = (ni as i32)-(self.tail.0 as i32);
        let delta_j = (nj as i32)-(self.tail.1 as i32);

        if delta_i.abs()>1 {
            if let Right=dir {
                self.tail = inc_pos(self.head, Left, self.size);
            } else if let Left=dir {
                self.tail = inc_pos(self.head, Right, self.size);
            }
        }

        if delta_j.abs()>1 {
            if let Up=dir {
                self.tail = inc_pos(self.head, Down, self.size);
            } else if let Down=dir {
                self.tail = inc_pos(self.head, Up, self.size);
            }
        }

        self.tail_trace.insert(self.tail);
    }

    fn play(&mut self, ins: Vec<Instruction>, visualise: bool) {
        for i in ins {
            if visualise { println!("== {} ==", i.to_string()); }
            let Instruction {dir, amount} = i;
            for _ in 0..amount {
                self.step(dir);
                if visualise {
                    self.draw_state();
                    println!();
                }
            }
        }
        self.calc_size();
    }
}

static INPUT_PATH : &str = "../input";
// static TEST_INPUT_PATH : &str = "../test_input";

fn main() {
    // let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let mut rope_sim = RopeSim::new((0,0), (0,0));

    let ins: Vec<_> = contents.lines().map(|l| l.parse::<Instruction>().unwrap()).collect();
    
    rope_sim.play(ins, false);
    rope_sim.draw_tail_trace();

    dbg!(rope_sim.tail_trace.len());

}
