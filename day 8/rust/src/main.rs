use std::{fs, str::FromStr};
use itertools::iproduct;

#[derive(Debug)]
struct Grid {
    values: Vec<Vec<u32>>,
    n: usize,
    m: usize
}

#[derive(Clone, Copy)]
pub enum CompassDirection {
    North,
    South,
    East,
    West
}

impl CompassDirection {
    const ALL : [Self; 4] = [Self::North, Self::South, Self::East, Self::West];

    pub fn iter() -> impl Iterator<Item=CompassDirection> {
        Self::ALL.iter().copied()
    }
}

impl FromStr for Grid {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = Vec::from_iter(s.lines().map(|line| {
            Vec::from_iter(line.chars().map(|c| c.to_digit(10).expect("File contains non-digit chars")))
        }));
        let n = values.len();
        let m = values[0].len();
        Ok( Grid {values, n, m} )
    }
}

impl Grid {
    fn pos_hidden_from(&self, (i, j): (usize, usize), dir: CompassDirection) -> bool {
        let v = self.values[i][j];
        if i==0 || i==self.n-1 || j==0 || j==self.m-1 {
            return false;
        }
        match dir {
            CompassDirection::North => self.values[..i].iter().find(|r| r[j]>=v).is_some(),
            CompassDirection::South => self.values[i+1..].iter().find(|r| r[j]>=v).is_some(),
            CompassDirection::East  => self.values[i][j+1..].iter().find(|&&vo| vo>=v).is_some(),
            CompassDirection::West  => self.values[i][..j].iter().find(|&&vo| vo>=v).is_some()
        }
    }

    fn pos_not_hidden(&self, (i, j): (usize, usize)) -> bool {
        CompassDirection::iter().find(|dir| self.pos_hidden_from((i,j), *dir)==false).is_some()
    }

    fn num_visible(&self) -> usize {
        iproduct!(0..self.n, 0..self.m).filter(|&pos| self.pos_not_hidden(pos)).count()
    }
}

static INPUT_PATH : &str = "../input";
static TEST_INPUT_PATH : &str = "../test_input";

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn sanity_check() {
        let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
        let grid: Grid = tcontents.parse().unwrap();

        let expected = [false, true, true, false];
        for (i, dir) in CompassDirection::iter().enumerate() {
            assert_eq!(grid.pos_hidden_from((1,1), dir), expected[i]);
        }

        assert_eq!(grid.num_visible(), 21);
    }
}

fn main() {
    // let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let grid: Grid = contents.parse().unwrap();

    // for i in 0..grid.n {
    //     for j in 0..grid.m {
    //         print!("{}: {}  ", grid.values[i][j], if grid.pos_not_hidden((i,j)) {"T"} else {"F"});
    //     }
    //     println!();
    // }

    // let pos = (2,0);
    // for dir in CompassDirection::iter() {
    //     dbg!(grid.pos_hidden_from(pos, dir));
    // }

    println!("Num visible: {}", grid.num_visible());

}
