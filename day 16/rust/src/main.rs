use crossterm::{
    cursor::{Hide, Show},
    ExecutableCommand,
};
use regex::Regex;
use std::{collections::HashMap, fs, io::stdout, time::Instant};

mod pathfind;
use pathfind::FloodFill;

#[macro_use]
extern crate lazy_static;

struct Valve {
    name: String,
    rate: u32,
    connections: Vec<usize>,
}

fn parse_valve(line: &str) -> (String, u32, Vec<String>) {
    lazy_static! {
        static ref REG: Regex = Regex::new(
            r"Valve (.+) has flow rate=(.+); tunnels{0,1} leads{0,1} to valves{0,1} (.+)$"
        )
        .expect("Regex compile failure");
    }

    let name: String;
    let rate: u32;
    let conn: String;
    if let Some(cap) = REG.captures(line) {
        name = cap.get(1).unwrap().as_str().parse().unwrap();
        rate = cap.get(2).unwrap().as_str().parse().unwrap();
        conn = cap.get(3).unwrap().as_str().parse().unwrap();
    } else {
        panic!("Regex match failure on line:\n{line}");
    }

    (
        name,
        rate,
        conn.split(", ").map(|s| String::from(s)).collect(),
    )
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
        let mut hist = Vec::with_capacity(32);
        hist.push(Action::Move(start_idx));
        State {
            minute: 0,
            total_rate: 0,
            released_pressure: 0,
            currently_at: start_idx,
            opened: vec![false; n],
            hist,
        }
    }

    fn next(&self) -> Self {
        let mut new = self.clone();
        new.minute += 1;
        new.released_pressure += self.total_rate;
        new
    }

    fn forms_cycle(&self, next_valve: usize) -> bool {
        if let Some((last_occured, _)) = self.hist.iter().enumerate().rev().find(|(_, a)| match a {
            Action::Move(valve) => *valve == next_valve,
            _ => false,
        }) {
            if let Some(_) = (last_occured + 1..self.hist.len()).find(|i| match self.hist[*i] {
                Action::Open => true,
                _ => false,
            }) {
                return false;
            } else {
                return true;
            }
        }
        false
    }

    /// Returns possible next actions
    fn insert_future(&self, valves: &Vec<Valve>, next_actions: &mut Vec<State>) {
        if self.minute >= 30 {
            return;
        }

        // travel to neighbouring room
        for &neighbour_idx in valves[self.currently_at].connections.iter() {
            if !self.forms_cycle(neighbour_idx) {
                let mut new = self.next();
                new.currently_at = neighbour_idx;
                new.hist.push(Action::Move(neighbour_idx));
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

fn state_string(state: &State, valves: &Vec<Valve>) -> String {
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
    let parsed_valves: Vec<_> = Vec::from_iter(contents.lines().map(parse_valve));
    let n_valves = parsed_valves.len();
    let mut valves = vec![];

    let get_idx = |k: &String| {
        parsed_valves
            .iter()
            .enumerate()
            .find(|(_, (name, _, _))| *name == *k)
            .unwrap()
            .0
    };
    for (name, rate, str_conn) in parsed_valves.iter() {
        let connections: Vec<usize> = Vec::from_iter(str_conn.iter().map(get_idx));
        valves.push(Valve {
            name: name.clone(),
            rate: *rate,
            connections,
        })
    }

    let start_idx = get_idx(&String::from("AA"));
    let mut queue: Vec<State> = vec![State::starting(start_idx, n_valves)];
    let mut best = State::starting(start_idx, n_valves);

    // brute action tree search
    let mut i: u64 = 0;
    let maxsecs = 600;
    let start = Instant::now();
    stdout().execute(Hide).unwrap();
    while let Some(current) = queue.pop() {
        current.insert_future(&valves, &mut queue);

        if current.released_pressure > best.released_pressure {
            best = current;
        }

        i += 1;
        if i % 10000 == 0 {
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

    println!("\n{}", state_string(&best, &valves));

    for (i, o) in best.opened.iter().enumerate() {
        if valves[i].rate > 0 {
            let n = if *o { "" } else { "not " };
            println!("{} ({}) {}opened", valves[i].name, valves[i].rate, n);
        }
    }

    if queue.len() > 0 {
        println!("\nWarning: search terminated early.");
    }
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    // part1(&contents);

    let parsed_valves: Vec<_> = Vec::from_iter(tcontents.lines().map(parse_valve));
    let n_valves = parsed_valves.len();
    let mut valves = vec![];

    let get_idx = |k: &String| {
        parsed_valves
            .iter()
            .enumerate()
            .find(|(_, (name, _, _))| *name == *k)
            .unwrap()
            .0
    };
    for (name, rate, str_conn) in parsed_valves.iter() {
        let connections: Vec<usize> = Vec::from_iter(str_conn.iter().map(get_idx));
        valves.push(Valve {
            name: name.clone(),
            rate: *rate,
            connections,
        })
    }

    let edges = HashMap::from_iter((0..n_valves).map(|i| (i, valves[i].connections.clone())));

    let ff_from_aa = FloodFill::new(0..n_valves, get_idx(&String::from("AA")), &edges);

    dbg!(ff_from_aa
        .path_to(get_idx(&String::from("HH")))
        .iter()
        .map(|&i| valves[i].name.clone())
        .collect::<Vec<_>>());
}
