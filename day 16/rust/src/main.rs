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

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    rate: u32,
    all_connections: Vec<usize>,
    relevant_connections: Vec<(usize, u32)>,
}

fn parse_valve(line: &str) -> (&str, u32, Vec<&str>) {
    lazy_static! {
        static ref REG: Regex = Regex::new(
            r"Valve (.+) has flow rate=(.+); tunnels{0,1} leads{0,1} to valves{0,1} (.+)$"
        )
        .expect("Regex compile failure");
    }

    let name: &str;
    let rate: u32;
    let conn: &str;
    if let Some(cap) = REG.captures(line) {
        name = cap.get(1).unwrap().as_str();
        rate = cap.get(2).unwrap().as_str().parse().unwrap();
        conn = cap.get(3).unwrap().as_str();
    } else {
        panic!("Regex match failure on line:\n{line}");
    }

    (name, rate, conn.split(", ").collect())
}

struct Valves<'a> {
    all: Vec<Valve<'a>>,
    ids: HashMap<&'a str, usize>,
    num: usize,
    start_idx: usize,
}

impl<'a> Valves<'a> {
    fn from(contents: &'a String) -> Self {
        let parsed_valves = Vec::from_iter(contents.lines().map(parse_valve));
        let mut all = vec![];
        let num = parsed_valves.len();

        let ids = HashMap::from_iter(
            parsed_valves
                .iter()
                .enumerate()
                .map(|(i, (name, _, _))| (*name, i)),
        );
        let start_idx = ids["AA"];

        for (name, rate, str_conn) in parsed_valves.iter() {
            let all_connections: Vec<usize> = Vec::from_iter(str_conn.iter().map(|to| ids[*to]));
            all.push(Valve {
                name: *name,
                rate: *rate,
                all_connections,
                relevant_connections: vec![],
            })
        }

        Self {
            all,
            ids,
            num,
            start_idx,
        }
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
        for &neighbour_idx in valves[self.currently_at].all_connections.iter() {
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

fn part1(valves: &Valves) {
    let mut queue: Vec<State> = vec![State::starting(valves.start_idx, valves.num)];
    let mut best = State::starting(valves.start_idx, valves.num);

    // brute action tree search
    let mut i: u64 = 0;
    let maxsecs = 600;
    let start = Instant::now();
    stdout().execute(Hide).unwrap();
    while let Some(current) = queue.pop() {
        current.insert_future(&valves.all, &mut queue);

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
        if start.elapsed().as_secs() > maxsecs {
            break;
        }
    }
    stdout().execute(Show).unwrap();

    println!("\n{}", state_string(&best, &valves.all));

    for (i, o) in best.opened.iter().enumerate() {
        if valves.all[i].rate > 0 {
            let n = if *o { "" } else { "not " };
            println!(
                "{} ({}) {}opened",
                valves.all[i].name, valves.all[i].rate, n
            );
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

    let valves = Valves::from(&tcontents);
    // part1(&valves);

    let edges: HashMap<usize, Vec<usize>> =
        HashMap::from_iter((0..valves.num).map(|i| (i, valves.all[i].all_connections.clone())));

    let ff_from_aa = FloodFill::new(0..valves.num, valves.start_idx, &edges);

    let target = valves.ids["HH"];
    dbg!(ff_from_aa
        .path_to(target)
        .iter()
        .map(|&i| valves.all[i].name)
        .collect::<Vec<_>>());
    dbg!(ff_from_aa.dist(target));
}
