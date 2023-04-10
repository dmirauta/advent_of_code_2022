use std::{fs, collections::{BTreeSet}};

static ALPHABET : &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priority(c : char) -> usize {
    ALPHABET.find(c).expect("{c} not alphabetic?") as usize + 1
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

fn line_score(line : &str) -> usize {
    let n = line.len();
    let strings = vec![&line[0 .. n/2], &line[n/2 .. n]];
    unique_common_chars(strings)
            .iter().map(|&c| priority(c)).sum()

}

fn part1(contents : &String) {
    let duplicate_sum : usize = contents.lines().map(line_score).sum();
    dbg!(duplicate_sum);
}

fn part2(contents : &String) {
    let line_vec: Vec<_> = contents.lines().collect();
    let badge_sum : usize = line_vec.chunks(3).map(|triplet| 
                        priority(unique_common_char(triplet.to_vec()))).sum();
    dbg!(badge_sum);
}

#[cfg(test)]
mod tests {
    use std::{vec, fs};
    use crate::ALPHABET;
    static TEST_PATH : &str = "../test_input";

    #[test]
    fn part1() {
        let contents = fs::read_to_string(TEST_PATH).expect("Could not read {TEST_PATH}");
        let results = [16, 38, 42, 22, 20, 19];
        for (i, line) in contents.lines().enumerate() {
            let score = crate::line_score(line);
            println!("{}, {} ({})", line, score, ALPHABET.chars().nth(score-1).unwrap());
            assert_eq!(score, results[i])
        }
    }

    #[test]
    fn part2() {
        let contents = fs::read_to_string(TEST_PATH).expect("Could not read {TEST_PATH}");
        let line_vec: Vec<_> = contents.lines().collect();
        let results = [18, 52];
        for (i, trip) in line_vec.chunks(3).enumerate() {
            let score = crate::priority(crate::unique_common_char(trip.to_vec()));
            println!("{} ({})", score, ALPHABET.chars().nth(score-1).unwrap());
            assert_eq!(score, results[i]);
        }
    }

}

static INPUT_PATH : &str = "../input";

fn main() {
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    part1(&contents);
    part2(&contents);

}
