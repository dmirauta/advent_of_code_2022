use std::{str::FromStr, fs, collections::{HashSet}};

type Pos = (i32, i32);

#[derive(Debug)]
struct RopeSim {
    head: Pos,
    tail: Pos,
    bottom_left: Pos,
    top_right: Pos,
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
        let mut split = s.split_whitespace();
        let dir: Direction = split.next().unwrap().chars().next().unwrap().try_into().unwrap();
        let amount: usize = split.next().unwrap().parse().unwrap();
        Ok( Instruction { dir, amount } )
    }
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        let c = match self.dir {
            Left  => 'L',
            Right => 'R',
            Down  => 'D',
            Up    => 'U'
        };
        format!("{} {}", c, self.amount)
    }
}

fn inc_pos((i,j):Pos, dir: Direction) -> Pos {
    match dir {
        Left  => (i-1, j),
        Right => (i+1, j),
        Up    => (i, j+1),
        Down  => (i, j-1)
    }
}

impl RopeSim {
    fn new() -> Self {
        RopeSim { head:(0,0), tail:(0,0), bottom_left:(0,0), top_right:(1,1), tail_trace:HashSet::new()}
    }

    fn calc_extents(&mut self) {
        let (i_min, _) = self.tail_trace.iter().min_by_key(|(i,_)| i).unwrap();
        let (_, j_min) = self.tail_trace.iter().min_by_key(|(_,j)| j).unwrap();

        let (i_max, _) = self.tail_trace.iter().max_by_key(|(i,_)| i).unwrap();
        let (_, j_max) = self.tail_trace.iter().max_by_key(|(_,j)| j).unwrap();

        self.bottom_left = (*i_min, *j_min);
        self.top_right = (*i_max+1, *j_max+1);
    }

    fn draw_state(&self) {
        let (left, bot) = self.bottom_left;
        let (right, top) = self.top_right;
        for j in (bot..top).rev() {
            for i in left..right {
                let c = if (i,j)==self.head { "H" } 
                         else if (i,j)==self.tail { "T" } 
                         else { "." };
                print!("{c}");
            }
            println!();
        }
    }

    fn draw_tail_trace(&self) {
        let (left, bot) = self.bottom_left;
        let (right, top) = self.top_right;
        for j in (bot..top).rev() {
            for i in left..right {
                let c = if self.tail_trace.contains(&(i,j)) { "#" }
                         else { "." };
                print!("{c}");
            }
            println!();
        }
    }

    fn step(&mut self, dir: Direction) {
        self.head = inc_pos(self.head, dir);
        
        let (hi, hj) = self.head;
        let (ti, tj) = self.tail;

        let delta_i = hi-ti;
        let delta_j = hj-tj;

        if delta_i.abs()>1 {
            if let Right=dir {
                self.tail = inc_pos(self.head, Left);
            } else if let Left=dir {
                self.tail = inc_pos(self.head, Right);
            }
        }

        if delta_j.abs()>1 {
            if let Up=dir {
                self.tail = inc_pos(self.head, Down);
            } else if let Down=dir {
                self.tail = inc_pos(self.head, Up);
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
        self.calc_extents();
    }
}

static INPUT_PATH : &str = "../input";
// static TEST_INPUT_PATH : &str = "../test_input";

fn main() {
    // let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let mut rope_sim = RopeSim::new();

    let ins: Vec<_> = contents.lines().map(|l| l.parse::<Instruction>().unwrap()).collect();
    
    rope_sim.play(ins, false);
    rope_sim.draw_tail_trace();

    dbg!(rope_sim.tail_trace.len());

}
