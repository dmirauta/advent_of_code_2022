use crossterm::{cursor, ExecutableCommand};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fs,
    io::stdout,
    ops::RangeInclusive,
    time::Instant,
};

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
struct AgentState {
    currently_at: usize,
    plan: VecDeque<Action>,
    hist: Vec<Action>,
}

impl AgentState {
    fn new(start_idx: usize) -> Self {
        let mut hist = Vec::with_capacity(32);
        hist.push(Action::Move(start_idx));
        Self {
            currently_at: start_idx,
            plan: VecDeque::new(),
            hist,
        }
    }

    fn transition(&self, action: Action) -> Self {
        let mut next = self.clone();
        if let Action::Move(destination_idx) = action {
            next.currently_at = destination_idx;
        }
        next.hist.push(action);
        next
    }

    /// All possible transitions
    fn future(&mut self, valves: &Valves, remaining_major: Vec<usize>) -> Vec<Self> {
        if let Some(action) = self.plan.pop_front() {
            return vec![self.transition(action)];
        }

        if remaining_major.len() == 0 {
            return vec![self.transition(Action::Stay)];
        }

        return remaining_major
            .iter()
            .map(|&destination_idx| {
                let path = valves.floodfills[&self.currently_at].path_to(destination_idx);

                let mut next = self.transition(Action::Move(path[1]));
                for &future_dest in path[2..].iter() {
                    next.plan.push_back(Action::Move(future_dest))
                }
                next.plan.push_back(Action::Open); // Open destination
                next
            })
            .collect();
    }
}

#[derive(Debug, Clone)]
struct NetworkState {
    minute: u32,
    total_rate: u32,
    released_pressure: u32,
    opened: Vec<bool>,
    agents: Vec<AgentState>,
}

impl NetworkState {
    fn starting(n: usize, agents: Vec<AgentState>) -> Self {
        Self {
            minute: 0,
            total_rate: 0,
            released_pressure: 0,
            opened: vec![false; n],
            agents,
        }
    }

    fn transition(&self, new_agent_states: Vec<AgentState>, valves: &Valves) -> Self {
        let mut next = self.clone();
        next.minute += 1;
        next.released_pressure += self.total_rate;
        next.agents = new_agent_states;
        for agent in next.agents.iter() {
            if let Some(Action::Open) = agent.hist.last() {
                next.opened[agent.currently_at] = true;
                next.total_rate += valves.all[agent.currently_at].rate;
            }
        }
        next
    }

    /// All possible transitions
    fn future(&mut self, valves: &Valves) -> Vec<NetworkState> {
        let remaining_major: Vec<usize> = valves
            .major
            .iter()
            .filter(|ni| !self.opened[**ni])
            .map(|ni| ni.clone())
            .collect();
        // TODO: multi agent
        self.agents[0]
            .future(valves, remaining_major)
            .iter()
            .map(|agent| self.transition(vec![agent.clone()], valves))
            .collect()
    }

    fn replay(&self, valves: &Vec<Valve>, time: RangeInclusive<usize>) {
        let mut previous = 0;
        let mut total_rate = 0;
        let mut total_released = 0;
        for i in time {
            println!("\n== Minute {} ==", i);
            println!("Releasing {total_rate} pressure.");
            total_released += total_rate;
            println!("{total_released} total.");
            for (j, agent) in self.agents.iter().enumerate() {
                let current = agent.hist[i].clone();
                print!("Agent {} ", j + 1);
                match current {
                    Action::Move(id) => {
                        previous = id;
                        println!("moves to {}", valves[id].name);
                    }
                    Action::Open => {
                        total_rate += valves[previous].rate;
                        println!("opens {}", valves[previous].name);
                    }
                    Action::Stay => println!("stays"),
                };
            }
        }
    }

    fn search_for_best_action_sequence(valves: &Valves, agents: Vec<AgentState>, max_time: u32) {
        let mut best = NetworkState::starting(valves.num, agents);
        let mut queue: Vec<NetworkState> = vec![best.clone()];

        let mut i: u64 = 0;
        let maxsecs = 60;
        let start = Instant::now();
        stdout().execute(cursor::Hide).unwrap();
        while let Some(mut current) = queue.pop() {
            if current.minute < max_time {
                queue.append(&mut current.future(&valves));
            }

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
            if start.elapsed().as_secs() > maxsecs {
                break;
            }
        }
        stdout().execute(cursor::Show).unwrap();
        println!();

        best.replay(&valves.all, 1..=(max_time as usize));

        // for (i, o) in best.opened.iter().enumerate() {
        //     if valves.all[i].rate > 0 {
        //         let n = if *o { "" } else { "not " };
        //         println!(
        //             "{} ({}) {}opened",
        //             valves.all[i].name, valves.all[i].rate, n
        //         );
        //     }
        // }

        if queue.len() > 0 {
            println!("\nWarning: search terminated early.");
        }
    }
}

fn part1(valves: &Valves) {
    NetworkState::search_for_best_action_sequence(
        valves,
        vec![AgentState::new(valves.start_idx)],
        30,
    )
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let valves = Valves::from(&tcontents);
    // dbg!(&valves.start_idx);
    part1(&valves);

    let ff_from_aa = &valves.floodfills[&valves.start_idx];
    // let target = valves.ids["OF"];
    //
    // dbg!(ff_from_aa
    //     .path_to(target)
    //     .iter()
    //     .map(|&i| valves.all[i].name)
    //     .collect::<Vec<_>>());
    //
    // dbg!(ff_from_aa.dist(target));

    // for i in valves.major {
    //     dbg!(valves.all[i].name, ff_from_aa.dist(i));
    // }
}
