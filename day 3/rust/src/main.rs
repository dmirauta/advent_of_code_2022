use std::{fs as fs, collections::HashSet};

static ALPHABET : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priority(c : char) -> u32 {
    ALPHABET.find(c).expect("{c} not alphabetic?") as u32 + 1
}

fn find_duplicate_chars(s1: &str, s2: &str) -> HashSet<char> {
    let mut set = s1.chars().collect::<HashSet<char>>();
    set.retain(|&c| s2.contains(c));
    set
}

fn line_score(line : &str) -> u32 {
    let n = line.len();
    find_duplicate_chars(&line[0 .. n/2], &line[n/2 .. n])
            .iter().map(|&c| priority(c)).sum()

}

static FILE_PATH : &str = "../input";

fn main() {

    // println!("{:?}", line_score("vJrwpWtwJgWrhcsFMMfFFhFp"));
    // println!("{:?}", line_score("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
    // println!("{:?}", line_score("PmmdzqPrVvPwwTWBwg"));
    // println!("{:?}", line_score("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
    // println!("{:?}", line_score("ttgJtRGJQctTZtZT"));
    // println!("{:?}", line_score("CrZsJsPPZsGzwwsLwLmpwMDw"));
    
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read {FILE_PATH}");

    let mut sum = 0;
    for line in contents.lines() {
        sum += line_score(line);
    }

    println!("{sum}")

}
