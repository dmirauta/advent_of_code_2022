use std::{fs, str::FromStr};

#[derive(Debug)]
struct Grid(Vec<Vec<u32>>);

impl FromStr for Grid {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Vec::from_iter(s.lines().map(|line| {
            Vec::from_iter(line.chars().map(|c| c.to_digit(10).expect("File contains non-digit chars")))
        }));
        Ok( Grid(grid) )
    }
}

static INPUT_PATH : &str = "../input";
static TEST_INPUT_PATH : &str = "../test_input";

fn main() {
    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let grid: Grid = tcontents.parse().unwrap();

    dbg!(grid);
}
