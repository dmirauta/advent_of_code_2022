use std::{fs, collections::BTreeSet};

fn first_unique_packet(stream : &str, packet_size: usize) -> usize {
    for i in 0..stream.len()-packet_size {
        if BTreeSet::from_iter(stream[i..i+packet_size].chars()).len()==packet_size {
            return i+packet_size;
        }
    }
    println!("no packet found in loop");
    stream.len()+1
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;
    static TEST_INPUT_PATH : &str = "../test_input";

    #[test]
    fn part_1() {
        let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
        let results = [7,5,6,10,11];
        for (i, line) in tcontents.lines().enumerate() {
            assert_eq!(first_unique_packet(line, 4), results[i])
        }
    }

    #[test]
    fn part_2() {
        let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
        let results = [19, 23, 23, 29, 26];
        for (i, line) in tcontents.lines().enumerate() {
            assert_eq!(first_unique_packet(line, 14), results[i])
        }
    }

}

static INPUT_PATH : &str = "../input";

fn main() {
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    dbg!(first_unique_packet(contents.as_str(), 4));
    dbg!(first_unique_packet(contents.as_str(), 14));
    
}
