use std::{fs as fs};

static FILE_PATH : &str = "../input";

fn parse() -> Vec<u32> {
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read {FILE_PATH}");
    
    let mut calories:Vec<u32> = vec![0];

    for line in contents.lines() {
        if let Some(cal) = line.parse::<u32>().ok() {
            *calories.last_mut().unwrap() += cal;
        } else { // every time we encounter a line without an integer, we are about to look at a different elf
            calories.push(0) // lines does not include last empty line?
        }
    }

    calories
}

fn part1(calories : &Vec<u32>) {
    println!("Part 1:\n {}\n", calories.iter().max().unwrap())
}

fn part2(calories : &mut Vec<u32>) {
    calories.sort();
    calories.reverse();

    println!("Part 2:");
    println!(" top 3 -> {:?}", &calories[..3]);
    println!(" sum -> {:?}", calories[..3].iter().sum::<u32>());
}

fn main() {

    let mut calories = parse();

    part1(&calories);
    part2(&mut calories)

}
