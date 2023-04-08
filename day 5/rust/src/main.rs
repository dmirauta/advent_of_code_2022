use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

static INPUT_PATH : &str = "../input";
static TEST_INPUT_PATH : &str = "../test_input";

fn process(line: &str) -> Vec<Option<char>> {
    let mut crate_row = vec![];
    let mut lc = line.chars();
    while let (Some(c1), Some(c2), Some(c3), _) = (lc.next(),lc.next(),lc.next(),lc.next()) {
        if c1=='[' && c3==']' {
            crate_row.push(Some(c2));
        } else {
            crate_row.push(None);
        }
    }
    crate_row
}

fn main() {
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let mut tlines = tcontents.lines();

    dbg!(process(tlines.next().unwrap()));
    dbg!(process(tlines.next().unwrap()));
    dbg!(process(tlines.next().unwrap()));

}
