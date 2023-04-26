use std::{str::FromStr, fs, collections::HashSet};

type Pos = (i32, i32);

#[derive(Debug)]
struct RopeSim {
    head: Pos,
    tail: Vec<Pos>,
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

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Left  => 'L',
            Right => 'R',
            Down  => 'D',
            Up    => 'U'
        }
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        let c: char = (*self).into();
        c.to_string()
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
        format!("{} {}", self.dir.to_string(), self.amount)
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

fn follow((hi,hj):Pos, (ti,tj):Pos) -> Pos {
    let (di, dj) = (hi-ti, hj-tj);
    if di.abs()>1 && dj.abs()==0 {        // same row
        (ti + di.signum(), tj)
    } else if di.abs()==0 && dj.abs()>1 { // same column
        (ti, tj + dj.signum())
    } else if  di.abs()>1 && dj.abs()>=1  // knights move
           ||  dj.abs()>1 && di.abs()>=1 {
        (ti + di.signum(), tj + dj.signum())
    } else {
        (ti, tj)
    }
}

static KNOT_SYMBOLS : &str = "123456789";

impl RopeSim {
    fn new(length: usize) -> Self {
        let tail = vec![(0,0); length];
        RopeSim { head:(0,0), tail, bottom_left:(0,0), top_right:(1,1), tail_trace:HashSet::new()}
    }

    fn calc_extents(&mut self) {
        let (ti_min, _) = *self.tail_trace.iter().min_by_key(|(i,_)| i).unwrap();
        let (_, tj_min) = *self.tail_trace.iter().min_by_key(|(_,j)| j).unwrap();

        let (ti_max, _) = *self.tail_trace.iter().max_by_key(|(i,_)| i).unwrap();
        let (_, tj_max) = *self.tail_trace.iter().max_by_key(|(_,j)| j).unwrap();

        let (i_min, _) = *self.tail.iter().min_by_key(|(i,_)| i).unwrap();
        let (_, j_min) = *self.tail.iter().min_by_key(|(_,j)| j).unwrap();

        let (i_max, _) = *self.tail.iter().max_by_key(|(i,_)| i).unwrap();
        let (_, j_max) = *self.tail.iter().max_by_key(|(_,j)| j).unwrap();

        let (i,j) = self.head;
        self.bottom_left = (i.min(i_min).min(ti_min), j.min(j_min).min(tj_min));
        self.top_right   = (i.max(i_max).max(ti_max), j.max(j_max).max(tj_max));
    }

    fn draw_state(&self) {
        let (left, bot) = self.bottom_left;
        let (right, top) = self.top_right;
        for j in (bot..=top).rev() {
            for i in left..=right {
                let mut c = '.';
                for (k, tp) in self.tail.iter().enumerate().rev() {
                    if (i,j)==*tp {
                        c = KNOT_SYMBOLS.chars().nth(k).unwrap();
                    }
                }
                if (i,j)==self.head {
                    c = 'H';
                }
                print!("{c}");
            }
            println!();
        }
    }

    fn draw_tail_trace(&self) {
        let (left, bot) = self.bottom_left;
        let (right, top) = self.top_right;
        let pos_to_char = |pos| if self.tail_trace.contains(&pos) { '#' } else { '.' };
        for j in (bot..=top).rev() {
            for i in left..=right {
                print!("{}", pos_to_char((i,j)));
            }
            println!();
        }
    }

    fn step(&mut self, dir: Direction) {
        self.head = inc_pos(self.head, dir);
        
        self.tail[0] = follow(self.head, self.tail[0]);
        for i in 1..self.tail.len() {
            self.tail[i] = follow(self.tail[i-1], self.tail[i]);
        }

        self.tail_trace.insert(*self.tail.last().unwrap());
    }

    fn play(&mut self, ins: Vec<Instruction>, visualise: bool) {
        for i in ins {
            if visualise { println!("== {} ==", i.to_string()); }
            let Instruction {dir, amount} = i;
            for _ in 0..amount {
                self.step(dir);
                if visualise {
                    self.calc_extents();
                    self.draw_state();
                    // println!("{:?}", self.tail.iter().format(" "));
                    println!();
                }
            }
        }
        self.calc_extents();
    }
}

static INPUT_PATH : &str = "../input";
static TEST_INPUT_PATH : &str = "../test_input";
static TEST_INPUT_PATH2 : &str = "../test_input2";

fn main() {
    // let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    // let tcontents2 = fs::read_to_string(TEST_INPUT_PATH2).expect("Could not read {TEST_INPUT_PATH2}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let mut rope_sim = RopeSim::new(9);

    let ins: Vec<_> = contents.lines().map(|l| l.parse::<Instruction>().unwrap()).collect();
    
    rope_sim.play(ins, false);
    // rope_sim.draw_tail_trace();

    dbg!(rope_sim.tail_trace.len());
    // println!("{:?}", rope_sim.tail_trace.iter().format(" "));
    // dbg!(rope_sim.bottom_left);
    // dbg!(rope_sim.top_right);

}
