use std::{fs, collections::BTreeSet};

fn first_unique_packet(stream : &str, packet_size: usize) -> usize {
    let mut i: usize = 0;
    while BTreeSet::from_iter(stream[i..i+packet_size].chars()).len()!=packet_size {
        i+=1;
    }
    i+packet_size
}

static INPUT_PATH : &str = "../input";
// static TEST_INPUT_PATH : &str = "../test_input";

fn main() {
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    // let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");

    // for line in tcontents.lines() {
    //     dbg!(first_unique_packet(line, 4));
    // }

    dbg!(first_unique_packet(contents.as_str(), 4));
    dbg!(first_unique_packet(contents.as_str(), 14));
    
}
