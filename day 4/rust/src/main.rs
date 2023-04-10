use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    low: u32,
    high: u32,
}

impl Range {
    fn encloses(&self, other: &Self) -> bool {
        self.low <= other.low && other.high <= self.high
    }
    fn contains(&self, p: u32) -> bool {
        self.low <= p && p<= self.high
    }
    fn overlaps(&self, other: &Self) -> bool {
        if self.low < other.low {
            self.contains(other.low)
        } else {
            other.contains(self.low)
        }
    }
}

#[derive(Debug)]
enum RangeParseError {
    BadDigit,
    SplitFail
}

impl From<ParseIntError> for RangeParseError {
    fn from(_err: ParseIntError) -> Self {
        RangeParseError::BadDigit
    }
}

impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split("-");
        if let (Some(lstr), Some(hstr)) = (sp.next(), sp.next()) {
            Ok( Self{ low: lstr.parse()?, high: hstr.parse()? } )
        } else {
            Err( RangeParseError::SplitFail )
        }
    }
}

fn range_pair(s: &str) -> Option<(Range, Range)> {
    let mut sp = s.split(",");
    if let (Some(r1s), Some(r2s)) = (sp.next(), sp.next()) {
        if let (Some(r1), Some(r2)) = (r1s.parse::<Range>().ok(), r2s.parse::<Range>().ok()) {
            return Some( (r1, r2) );
        }
    }
    println!("invalid line encountered");
    return None;
}

fn line_has_enclosing(line: &str) -> bool {
    match range_pair(line) {
        Some((r1,r2)) => r1.encloses(&r2) || r2.encloses(&r1),
        None => false
    }
}

fn line_has_overlap(line: &str) -> bool {
    match range_pair(line) {
        Some((r1,r2)) => r1.overlaps(&r2),
        None => false
    }
}

static INPUT_PATH : &str = "../input";

#[cfg(test)]
mod test {
    use std::fs;
    use crate::*;
    static TEST_INPUT_PATH : &str = "../test_input";

    #[test]
    fn part1() {
        let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
        let results: [bool; 6] = [false, false, false, true, true, false];
        for (i, line) in tcontents.lines().enumerate() {
            assert_eq!(line_has_enclosing(line), results[i]);
        }
    }

    #[test]
    fn part2() {
        let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
        let results: [bool; 6] = [false, false, true, true, true, true];
        for (i, line) in tcontents.lines().enumerate() {
            assert_eq!(line_has_overlap(line), results[i]);
        }
    }
}

fn main() {
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let num_enclosings = contents.lines().filter(|l| line_has_enclosing(l)).count();
    dbg!(num_enclosings);

    let num_overlaps = contents.lines().filter(|l| line_has_overlap(l)).count();
    dbg!(num_overlaps);
}
