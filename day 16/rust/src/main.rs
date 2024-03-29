use crossterm::{cursor, ExecutableCommand};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::stdout,
    ops::RangeInclusive,
    time::Instant,
};

mod pathfind;
use pathfind::FloodFill;

extern crate rayon;

use rayon::prelude::*;

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

struct ValveNetwork<'a> {
    all: Vec<Valve<'a>>,
    major: Vec<usize>,
    ids: HashMap<&'a str, usize>,
    floodfills: HashMap<usize, FloodFill<usize>>,
    num: usize,
    start_idx: usize,
}

impl<'a> ValveNetwork<'a> {
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

    fn replay_sequence(&self, agents: &Vec<AgentState>, time: RangeInclusive<usize>) {
        let mut previous: HashMap<usize, usize> =
            HashMap::from_iter((0..agents.len()).map(|i| (i, 0)));
        let mut total_rate = 0;
        let mut total_released = 0;
        for i in time {
            println!("\n== Minute {} ==", i);
            println!("Releasing {total_rate} pressure.");
            total_released += total_rate;
            println!("{total_released} total.");
            for (j, agent) in agents.iter().enumerate() {
                let current = agent.hist[i - 1].clone();
                print!("Agent {} ", j + 1);
                match current {
                    Action::Move(id) => {
                        *previous.get_mut(&j).unwrap() = id;
                        println!("moves to {}", self.all[id].name);
                    }
                    Action::Open => {
                        total_rate += self.all[previous[&j]].rate;
                        println!("opens {}", self.all[previous[&j]].name);
                    }
                    Action::Stay => println!("stays"),
                };
            }
        }
    }

    fn search_for_best_action_sequence(&self, agents: Vec<AgentState>, max_sim_time: u32) {
        let init_queue: Vec<NetworkState> = NetworkState::starting(self.num, agents)
            .future(self)
            .iter()
            .filter(|initial_network_state| {
                if initial_network_state.agents.len() == 2 {
                    initial_network_state.agents[0].targeting.unwrap()
                        < initial_network_state.agents[1].targeting.unwrap()
                } else {
                    true
                }
            })
            .map(|v| v.clone())
            .collect();

        stdout().execute(cursor::Hide).unwrap();
        let best_per: Vec<_> = init_queue
            .par_iter()
            .enumerate()
            .map(|(i, start)| self.best_sequence_from(start.clone(), max_sim_time, i))
            .collect();
        stdout().execute(cursor::Show).unwrap();
        println!();

        let best = best_per
            .iter()
            .max_by(|a, b| a.released_pressure.cmp(&b.released_pressure))
            .unwrap();

        self.replay_sequence(&best.agents, 1..=(max_sim_time as usize));

        // for (i, o) in best.opened.iter().enumerate() {
        //     if valves.all[i].rate > 0 {
        //         let n = if *o { "" } else { "not " };
        //         println!(
        //             "{} ({}) {}opened",
        //             valves.all[i].name, valves.all[i].rate, n
        //         );
        //     }
        // }
    }

    fn best_sequence_from(&self, init: NetworkState, max_sim_time: u32, id: usize) -> NetworkState {
        let mut best = init.clone();
        let mut queue: Vec<NetworkState> = vec![init];

        let mut expanded: u64 = 0;
        let max_real_time = 240;
        let start = Instant::now();

        while let Some(mut current) = queue.pop() {
            let secs_passed = start.elapsed().as_secs();
            if secs_passed > max_real_time {
                break;
            }
            if current.minute < max_sim_time {
                queue.append(&mut current.future(self));
            }
            if current.released_pressure > best.released_pressure {
                best = current;
            }
            if start.elapsed().as_millis() % 1000 == 0 || queue.len() < 2 {
                print!(
                    "(instance {id}) expanded = {expanded}, time = {secs_passed}, queue size = {}, best pressure released = {}     \r",
                    queue.len(),
                    best.released_pressure
                );
            }
            expanded += 1;
        }

        if queue.len() > 0 {
            println!("\nWarning: search terminated early.");
        }

        best
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
    targeting: Option<usize>,
    plan: VecDeque<Action>,
    hist: Vec<Action>,
}

impl AgentState {
    fn new(start_idx: usize) -> Self {
        Self {
            currently_at: start_idx,
            targeting: None,
            plan: VecDeque::with_capacity(32),
            hist: Vec::with_capacity(32),
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

    /// All possible (agent state) transitions
    fn future(&self, valves: &ValveNetwork, remaining_major: HashSet<usize>) -> Vec<Self> {
        if let Some(action) = self.plan.front() {
            let mut new = self.transition(action.clone());
            new.plan.pop_front();
            return vec![new];
        }

        if remaining_major.len() == 0 {
            let mut next = self.transition(Action::Stay);
            next.targeting = None;
            return vec![next];
        }

        remaining_major
            .iter()
            .map(|&destination_idx| {
                let path = &valves.floodfills[&self.currently_at].shortest_path[&destination_idx];

                let mut next = self.transition(Action::Move(path[1]));
                next.targeting = Some(destination_idx.clone());
                for &future_dest in path[2..].iter() {
                    next.plan.push_back(Action::Move(future_dest))
                }
                next.plan.push_back(Action::Open); // Open destination
                next
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct NetworkState {
    minute: u32,
    total_rate: u32,
    released_pressure: u32,
    targeted: Vec<bool>,
    opened: Vec<bool>,
    agents: Vec<AgentState>,
}

impl NetworkState {
    fn starting(n: usize, agents: Vec<AgentState>) -> Self {
        Self {
            minute: 0,
            total_rate: 0,
            released_pressure: 0,
            targeted: vec![false; n],
            opened: vec![false; n],
            agents,
        }
    }

    fn transition(&self) -> Self {
        Self {
            minute: self.minute + 1,
            total_rate: self.total_rate,
            released_pressure: self.released_pressure + self.total_rate,
            targeted: self.targeted.clone(),
            opened: self.opened.clone(),
            agents: vec![],
        }
    }

    fn add_agent(&mut self, agent: AgentState, valves: &ValveNetwork) {
        if let Some(t) = agent.targeting {
            self.targeted[t] = true;
        }
        if let Some(Action::Open) = agent.hist.last() {
            self.opened[agent.currently_at] = true;
            self.total_rate += valves.all[agent.currently_at].rate;
        }
        self.agents.push(agent);
    }

    fn remaining_targets(&self, valves: &ValveNetwork) -> HashSet<usize> {
        valves
            .major
            .iter()
            .filter(|ni| !self.targeted[**ni])
            .map(|ni| ni.clone())
            .collect()
    }

    /// All possible (network state) transitions
    fn future(&mut self, valves: &ValveNetwork) -> Vec<NetworkState> {
        let remaining_targets = self.remaining_targets(valves);

        // TODO: arbitrary&neat multi agent
        let mut network_states = vec![];
        match self.agents.len() {
            1 => {
                for agent_state in self.agents[0].future(valves, remaining_targets) {
                    let mut ns = self.transition();
                    ns.add_agent(agent_state, valves);
                    network_states.push(ns);
                }
            }

            2 => {
                for agent_1_state in self.agents[0].future(valves, remaining_targets.clone()) {
                    let mut ns = self.transition();
                    ns.add_agent(agent_1_state, valves);
                    for agent_2_state in self.agents[1].future(valves, ns.remaining_targets(valves))
                    {
                        let mut ns2 = ns.clone();
                        ns2.add_agent(agent_2_state, valves);
                        network_states.push(ns2);
                    }
                }
            }
            n => {
                panic!("unsuported number of agents ({n})")
            }
        }
        network_states
    }
}

fn part1(valves: &ValveNetwork) {
    valves.search_for_best_action_sequence(vec![AgentState::new(valves.start_idx)], 30)
}

fn part2(valves: &ValveNetwork) {
    valves.search_for_best_action_sequence(vec![AgentState::new(valves.start_idx); 2], 26)
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let valves = ValveNetwork::from(&contents);
    part2(&valves);
}
