use std::{fs as fs};

// positionally encoded wins, where moves[i%3] beats moves[(i-1)%3], e.g. "Y" [1] beats "A" [0]
static P1_MOVES : [char;3] = ['A', 'B', 'C'];
static P2_MOVES : [char;3] = ['X', 'Y', 'Z'];
                              // 0 rock
                                    // 1 paper
                                        // 2 scissors

static INPUT_PATH : &str = "../input";

fn parse() -> Vec<(char, char)> {
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");
    
    let mut moves:Vec<(char,char)> = vec![];

    for line in contents.lines() {
        let line_chars = Vec::from_iter(line.chars());
        if line.len() == 3 {
            // println!("{:?} {:?}", line_chars[0], line_chars[2]);
            moves.push((line_chars[0], line_chars[2]));
        }
    }

    moves
}

fn _score(p1_id : u32, p2_id : u32) -> u32 {
    // Start with move score, 1 for Rock (first pos), 2 for Paper...
    let mut score = 1 + p2_id;

    if p1_id == p2_id {
        score += 3
    } else if (p1_id +1)%3 == p2_id { // checking if p2 wins
        score += 6
    } // no extra points on loss

    score
}

fn score(p1_move : char, p2_move : char) -> u32 {
    // p1 is opponent, p2 is "you"

    let p1_id = P1_MOVES.iter().position(|&r| r == p1_move)
                        .expect("Invalid p1 move") as u32;
    let p2_id = P2_MOVES.iter().position(|&r| r == p2_move)
                        .expect("Invalid p2 move") as u32;

    _score(p1_id, p2_id)
}

fn score2(p1_move : char, outcome : char) -> u32 {

    let p1_id = P1_MOVES.iter().position(|&r| r == p1_move)
                        .expect("Invalid p1 move") as u32;
    let p2_offset = P2_MOVES.iter().position(|&r| r == outcome)
                        .expect("Invalid outcome") as u32 + 2; // -1 + 3
    let p2_id = (p1_id+p2_offset)%3;

    _score(p1_id, p2_id)
}

fn main() {
    let moves = parse();

    let total_score : u32 = moves.iter().map(|(m1,m2)| score(*m1, *m2)).sum();
    let total_score2 : u32 = moves.iter().map(|(m1,m2)| score2(*m1, *m2)).sum();

    println!("{:?} {:?}", total_score, total_score2);
    
}
