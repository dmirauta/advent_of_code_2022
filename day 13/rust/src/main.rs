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

fn part_1(contents: &String) -> usize {
    contents.split("\n\n").enumerate().fold(0, |s, (i, input)| {
        let mut sp = input.split("\n");
        let left: Node = sp.next().unwrap().trim().parse().unwrap();
        let right: Node = sp.next().unwrap().trim().parse().unwrap();
        if left < right {
            s + i + 1
        } else {
            s
        }
    })
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    // let contents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");
    dbg!(part_1(&contents));
}
