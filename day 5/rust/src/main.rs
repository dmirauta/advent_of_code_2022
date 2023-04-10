use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;


fn get_crates(line: &str) -> Vec<Option<char>> {
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

#[derive(Debug)]
struct Instruction {
    qty: usize,
    origin: usize, // 1 indexed as in text
    dest: usize    //
}

#[derive(Debug)]
enum InstructionParseError {
    BadDigit,
    SplitFail
}

impl From<ParseIntError> for InstructionParseError {
    fn from(_err: ParseIntError) -> Self {
        InstructionParseError::BadDigit
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp1 = s[5..].split(" from ");
        if let (Some(l), Some(r)) = (sp1.next(), sp1.next()) {
            let mut sp2 = r.split(" to ");
            if let (Some(m), Some(rr)) = (sp2.next(), sp2.next()) {
                return Ok( Self{ qty: l.parse()?, origin: m.parse()?, dest: rr.parse()? } );
            }   
        }
        return Err( InstructionParseError::SplitFail );
    }
}
type Stack = Vec<char>;
fn parse(contents: String) -> (Vec<Stack>, Vec<Instruction>) {
    let mut stacks: Vec<Stack> = vec![];
    let mut crate_rows: Vec<Vec<Option<char>>> = vec![];
    let mut lines = contents.lines();

    while let Some(l) = lines.next() {
        if !l.contains('[') {
            break; // consumes one non bracketed line (number line)
        }
        crate_rows.push(get_crates(l));
    } 

    for _ in 0..crate_rows[0].len() {
        stacks.push(vec![]);
    }

    crate_rows.reverse(); // we read from top, fill stacks from bottom
    for row in crate_rows {
        for (i, opt) in row.iter().enumerate() {
            if let Some(c) = opt {
                stacks[i].push(*c);
            }
        }
    }

    // parse move instructions
    lines.next(); // usually an empty line between crates and instructions?

    let instructions = Vec::from_iter(lines.map(|l| l.parse::<Instruction>().unwrap()));

    (stacks, instructions)

}

fn crate_mover9000(stacks: &mut Vec<Stack>, instructions: &Vec<Instruction>) {
    for &Instruction{qty, origin, dest} in instructions {
        for _ in 0..qty {
            if let Some(char) = stacks[origin-1].pop() {
                stacks[dest-1].push(char);
            } else {
                println!("Tried to pop empty stack");
            }
        }
    }
}

fn crate_mover9001(stacks: &mut Vec<Stack>, instructions: &Vec<Instruction>) {
    for &Instruction{qty, origin, dest} in instructions {
        let ministack: Vec<Option<char>> = (0..qty).map(|_| stacks[origin-1].pop()).collect();
        for m in ministack.iter().rev() {
            if let Some(char) = m {
                stacks[dest-1].push(*char);
            } else {
                println!("Tried to pop empty stack");
            }
        }
    }
}

fn stack_tops(stacks: &Vec<Stack>) -> String {
    String::from_iter(stacks.iter().map(|s| s.last().unwrap()))
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;
    static TEST_INPUT_PATH : &str = "../test_input";

    #[test]
    fn part_1() {
        let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
        let (mut stacks, instructions) = parse(tcontents);
        crate_mover9000(&mut stacks, &instructions);
        assert_eq!(stack_tops(&stacks), "CMZ");
    }

    #[test]
    fn part_2() {
        let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
        let (mut stacks, instructions) = parse(tcontents);
        crate_mover9001(&mut stacks, &instructions);
        assert_eq!(stack_tops(&stacks), "MCD");
    }
}

static INPUT_PATH : &str = "../input";

fn main() {
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let (mut stacks, instructions) = parse(contents);
    let mut stacks_p2 = stacks.clone();

    crate_mover9000(&mut stacks, &instructions);
    let part_1 = stack_tops(&stacks);
    dbg!(part_1);

    crate_mover9001(&mut stacks_p2, &instructions);
    let part_2 = stack_tops(&stacks_p2);
    dbg!(part_2);

}
