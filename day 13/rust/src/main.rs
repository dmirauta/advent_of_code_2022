// very much like https://fasterthanli.me/series/advent-of-code-2022/part-13

use std::{
    cmp::Ordering,
    fmt::{self, Display},
    fs,
    str::FromStr,
};

use serde_json;

#[derive(PartialEq, Eq)]
enum Node {
    Sublist(Vec<Node>),
    Value(u64),
}

impl Clone for Node {
    fn clone(&self) -> Self {
        match self {
            Self::Sublist(sl) => Self::Sublist(Vec::from_iter(sl.iter().map(|n| n.clone()))),
            Self::Value(v) => Self::Value(v.clone()),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(n) => write!(f, "{n}"),
            Self::Sublist(sl) => {
                let mut string = String::new();
                for val in sl {
                    string += format!("{} ", val).as_str();
                }
                write!(f, "[ {}]", string)
            }
        }
    }
}

impl From<serde_json::Value> for Node {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Number(n) => Node::Value(n.as_u64().unwrap()),
            serde_json::Value::Array(vec) => Node::Sublist(
                vec.iter()
                    .map(|val| Node::from(val.clone()))
                    .collect::<Vec<_>>(),
            ),
            _ => panic!("Invalid conversion."),
        }
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str::<serde_json::Value>(s) {
            Ok(v) => Ok(Node::from(v)),
            _ => Err(String::from("serde value parse error")),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Node::Value(n), Node::Value(no)) => n.cmp(&no),
            (Node::Sublist(sl), Node::Sublist(slo)) => sl
                .iter()
                .zip(slo.iter())
                .map(|(n, no)| n.cmp(no))
                .find(|ord| *ord != Ordering::Equal)
                .unwrap_or_else(|| sl.len().cmp(&slo.len())),
            (Node::Value(n), sl) => {
                let new_node = Node::Sublist(vec![Node::Value(n.clone())]);
                new_node.cmp(sl)
            }
            (sl, Node::Value(n)) => {
                let new_node = Node::Sublist(vec![Node::Value(n.clone())]);
                sl.cmp(&new_node)
            }
        }
    }
}

// derived partial ordering can be wrong?
// https://github.com/rust-lang/rust-clippy/issues/1621
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = self.cmp(other);

        // // debug
        // let cmp = match res {
        //     Ordering::Less => "<",
        //     Ordering::Equal => "=",
        //     Ordering::Greater => ">",
        // };
        // println!("{} {} {}", self, cmp, other);

        Some(res)
    }
}

fn part_1(packets: &Vec<Node>) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .fold(0, |s, (i, packet_slice)| {
            if packet_slice[0] < packet_slice[1] {
                s + i + 1
            } else {
                s
            }
        })
}

fn part_2(packets: &mut Vec<Node>) -> usize {
    let divider_1 = "[[2]]".parse::<Node>().unwrap();
    let divider_2 = "[[6]]".parse::<Node>().unwrap();
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort();
    let (i1, _) = packets
        .iter()
        .enumerate()
        .find(|(_, n)| **n == divider_1)
        .unwrap();
    let (i2, _) = packets
        .iter()
        .enumerate()
        .find(|(_, n)| **n == divider_2)
        .unwrap();

    (i1 + 1) * (i2 + 1)
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    // let contents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let mut packets: Vec<_> = contents
        .replace("\n\n", "\n")
        .lines()
        .filter_map(|l| l.trim().parse::<Node>().ok())
        .collect();

    dbg!(part_1(&packets));
    dbg!(part_2(&mut packets));
}
