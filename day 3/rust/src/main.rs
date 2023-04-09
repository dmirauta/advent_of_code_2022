use std::{fs as fs, collections::{BTreeSet}};

static ALPHABET : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priority(c : char) -> u32 {
    ALPHABET.find(c).expect("{c} not alphabetic?") as u32 + 1
}

// chars shared by all input strings
fn unique_common_chars(strings: Vec<&str>) -> BTreeSet<char> {
    let mut set = strings[0].chars().collect::<BTreeSet<char>>();
    for i in  1..strings.len() {
        set.retain(|&c| strings[i].contains(c));
    }
    set
}

fn unique_common_char(strings: Vec<&str>) -> char {
    unique_common_chars(strings).pop_last().unwrap()
}

fn line_score(line : &str) -> u32 {
    let n = line.len();
    let strings = vec![&line[0 .. n/2], &line[n/2 .. n]];
    unique_common_chars(strings)
            .iter().map(|&c| priority(c)).sum()

}

fn _part1_tests(contents : &String) {
    for line in contents.lines() {
        println!("{}, {}", line, line_score(line));
    }
}

fn part1(contents : &String) {
    let duplicate_sum : u32 = contents.lines().map(line_score).sum();
    dbg!(duplicate_sum);
}

fn _part2_tests(contents : &String) {
    let line_vec: Vec<_> = contents.lines().collect();
    for trip in line_vec.chunks(3) {
        dbg!(unique_common_chars(trip.to_vec()));
    }
}

fn part2(contents : &String) {
    let line_vec: Vec<_> = contents.lines().collect();
    let badge_sum : u32 = line_vec.chunks(3).map(|triplet| 
                        priority(unique_common_char(triplet.to_vec()))).sum();
    dbg!(badge_sum);
}

static INPUT_PATH : &str = "../input";
// static TEST_PATH : &str = "../test_input";

fn main() {
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");
    // let tcontents = fs::read_to_string(TEST_PATH).expect("Could not read {TEST_PATH}");

    // _part1_tests(&tcontents);
    part1(&contents);
    // _part2_tests(&tcontents);
    part2(&contents);

}
