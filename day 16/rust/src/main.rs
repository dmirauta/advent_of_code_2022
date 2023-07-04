use crossterm::{cursor, ExecutableCommand};
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
    connections: Vec<usize>,
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
    major: Vec<usize>,
    ids: HashMap<&'a str, usize>,
    floodfills: HashMap<usize, FloodFill<usize>>,
    num: usize,
    start_idx: usize,
}

impl<'a> Valves<'a> {
    fn from(contents: &'a String) -> Self {
        let parsed_valves = Vec::from_iter(contents.lines().map(parse_valve));
        let mut all = vec![];
        let mut major = vec![];
        let num = parsed_valves.len();

        let mut ids = HashMap::new();

        for (i, (name, rate, _)) in parsed_valves.iter().enumerate() {
            ids.insert(*name, i);
            if *rate > 0 {
                major.push(i);
            }
        }

        let start_idx = ids["AA"];

        for (name, rate, str_conn) in parsed_valves.iter() {
            let connections: Vec<usize> = Vec::from_iter(str_conn.iter().map(|to| ids[*to]));
            all.push(Valve {
                name: *name,
                rate: *rate,
                connections,
            })
        }

        let edges = (0..num).map(|i| (i, all[i].connections.clone())).collect();

        // we actually only need to pathfind from starting room or relevant_connections
        // mechanism for lazily computing these might be nice
        let floodfills = (0..num).map(|i| (i, FloodFill::new(i, &edges))).collect();

        Self {
            all,
            major,
            ids,
            floodfills,
            num,
            start_idx,
        }
    }
}

#[derive(Debug, Clone)]
enum Action {
    Move(usize),
    Open,
    Stay,
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

    fn next(&self, actions: Vec<Action>) -> Self {
        let mut new = self.clone();
        for action in actions {
            new.minute += 1;
            new.released_pressure += self.total_rate;
            new.hist.push(action);
        }
        new
    }

    /// Returns possible next actions
    fn insert_future(&self, valves: &Valves, next_actions: &mut Vec<State>) {
        if self.minute >= 30 {
            return;
        }

        let remaining_major: Vec<&usize> = valves
            .major
            .iter()
            .filter(|ni| !self.opened[**ni])
            .collect();

        if remaining_major.len() == 0 {
            next_actions.push(self.next(vec![Action::Stay]));
            return;
        }

        for &neighbour_idx in remaining_major {
            let path = valves.floodfills[&self.currently_at].path_to(neighbour_idx);
            let mut actions: Vec<_> = path[1..].iter().map(|i| Action::Move(*i)).collect();
            actions.push(Action::Open);

            let mut new = self.next(actions);
            new.currently_at = neighbour_idx;

            new.opened[neighbour_idx] = true;
            new.total_rate += valves.all[neighbour_idx].rate;

            if new.minute <= 30 {
                next_actions.push(new);
            }
        }
    }
}

fn state_string(state: &State, valves: &Vec<Valve>) -> String {
    let mut s = format!("released_pressure: {}\n", state.released_pressure);
    for action in state.hist.iter() {
        match action {
            Action::Move(id) => s += format!("Move to {}\n", valves[*id].name).as_str(),
            Action::Open => s += format!("Open\n").as_str(),
            Action::Stay => s += format!("Stay\n").as_str(),
        };
    }
    s
}

fn part1(valves: &Valves) {
    let mut best = State::starting(valves.start_idx, valves.num);
    let mut queue: Vec<State> = vec![best.clone()];

    let mut i: u64 = 0;
    let maxsecs = 60;
    let start = Instant::now();
    stdout().execute(cursor::Hide).unwrap();
    while let Some(current) = queue.pop() {
        current.insert_future(&valves, &mut queue);

        if current.released_pressure > best.released_pressure {
            best = current;
        }

        i += 1;
        if i % 100 == 0 {
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
    stdout().execute(cursor::Show).unwrap();

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

    let valves = Valves::from(&contents);
    // part1(&valves);

    let ff_from_aa = &valves.floodfills[&valves.start_idx];
    // let target = valves.ids["ZX"];
    //
    // dbg!(ff_from_aa
    //     .path_to(target)
    //     .iter()
    //     .map(|&i| valves.all[i].name)
    //     .collect::<Vec<_>>());
    //
    // dbg!(ff_from_aa.dist(target));

    for i in valves.major {
        dbg!(valves.all[i].name, ff_from_aa.dist(i));
    }
}
