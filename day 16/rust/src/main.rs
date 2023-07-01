use crossterm::{
    cursor::{Hide, Show},
    ExecutableCommand,
};
use regex::Regex;
use std::{fs, io::stdout, str::FromStr, time::Instant};

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct ParsedValve {
    name: String,
    rate: u32,
    connections: Vec<String>,
}

struct Valve {
    rate: u32,
    connections: Vec<usize>,
}

impl FromStr for ParsedValve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REG: Regex = Regex::new(
                r"Valve (.+) has flow rate=(.+); tunnels{0,1} leads{0,1} to valves{0,1} (.+)$"
            )
            .expect("Regex compile failure");
        }

        let name: String;
        let rate: u32;
        let conn: String;
        if let Some(cap) = REG.captures(s) {
            name = cap.get(1).unwrap().as_str().parse().unwrap();
            rate = cap.get(2).unwrap().as_str().parse().unwrap();
            conn = cap.get(3).unwrap().as_str().parse().unwrap();
        } else {
            return Err(String::from("Regex match failure"));
        }

        Ok(Self {
            name,
            rate,
            connections: conn.split(", ").map(|s| String::from(s)).collect(),
        })
    }
}

#[derive(Debug, Clone)]
enum Action {
    Move(usize),
    Open,
}

#[derive(Debug, Clone)]
struct State {
    minute: u32,
    total_rate: u32,
    released_pressure: u32,
    currently_at: usize,
    opened: Vec<bool>,
    hist: Vec<Action>,
}

impl State {
    fn starting(start_idx: usize, n: usize) -> Self {
        State {
            minute: 0,
            total_rate: 0,
            released_pressure: 0,
            currently_at: start_idx,
            opened: vec![false; n],
            hist: Vec::with_capacity(32),
        }
    }

    fn next(&self) -> Self {
        let mut new = self.clone();
        new.minute += 1;
        new.released_pressure += self.total_rate;
        new
    }

    /// Returns possible next actions
    fn insert_future(&self, valves: &Vec<Valve>, next_actions: &mut Vec<State>) {
        if self.minute >= 30 {
            return;
        }

        let last_visited: usize =
            if let Some(Action::Move(id)) = self.hist.get(self.hist.len().wrapping_sub(2)) {
                *id
            } else {
                usize::MAX
            };

        // travel to neighbouring room
        for neighbour_idx in valves[self.currently_at].connections.iter() {
            // avoid immediately cycling back
            if *neighbour_idx != last_visited {
                let mut new = self.next();
                new.currently_at = *neighbour_idx;
                new.hist.push(Action::Move(*neighbour_idx));
                next_actions.push(new);
            }
        }

        // open valve in current room
        if !self.opened[self.currently_at] && valves[self.currently_at].rate != 0 {
            let mut new = self.next();
            new.opened[self.currently_at] = true;
            new.total_rate += valves[self.currently_at].rate;
            new.hist.push(Action::Open);
            next_actions.push(new);
        }
    }
}

fn state_string(state: &State, valves: &Vec<ParsedValve>) -> String {
    let mut s = format!("released_pressure: {}\n", state.released_pressure);
    for action in state.hist.iter() {
        match action {
            Action::Move(id) => s += format!("{}\n", valves[*id].name).as_str(),
            Action::Open => s += format!("Open\n").as_str(),
        };
    }
    s
}

fn part1(contents: &String) {
    let parsed_valves: Vec<ParsedValve> =
        Vec::from_iter(contents.lines().map(|l| l.parse().unwrap()));
    let n_valves = parsed_valves.len();
    let mut valves = vec![];

    let get_idx = |k: &String| {
        parsed_valves
            .iter()
            .enumerate()
            .find(|(_, v)| v.name == *k)
            .unwrap()
            .0
    };
    for pv in parsed_valves.iter() {
        let connections: Vec<usize> = Vec::from_iter(pv.connections.iter().map(get_idx));
        valves.push(Valve {
            rate: pv.rate,
            connections,
        })
    }

    let start_idx = get_idx(&String::from("AA"));
    let mut queue: Vec<State> = vec![State::starting(start_idx, n_valves)];
    let mut best = State::starting(start_idx, n_valves);

    // brute action tree search
    let mut i: u64 = 0;
    let maxsecs = 20;
    let start = Instant::now();
    stdout().execute(Hide).unwrap();
    while queue.len() > 0 && start.elapsed().as_secs() < maxsecs {
        let current = queue.pop().unwrap();

        current.insert_future(&valves, &mut queue);

        if current.released_pressure > best.released_pressure {
            best = current;
        }

        i += 1;
        if i % 1000 == 0 {
            print!(
                "i = {}, time = {}, queue size = {}, best pressure released = {}     \r",
                &i,
                start.elapsed().as_secs(),
                queue.len(),
                best.released_pressure
            );
        }
    }
    stdout().execute(Show).unwrap();

    println!("\n{}", state_string(&best, &parsed_valves));
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    part1(&tcontents);
}
