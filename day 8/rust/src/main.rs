use std::{fs, str::FromStr};
use itertools::iproduct;

#[derive(Clone, Copy, Debug)]
pub enum CompassDirection {
    North,
    East,
    South,
    West
}
use CompassDirection::{North, East, South, West};

impl CompassDirection {
    pub fn iter() -> impl Iterator<Item=CompassDirection> {
        [North, East, South, West].iter().copied()
    }
}

#[derive(Debug)]
struct Grid {
    values: Vec<Vec<u8>>,
    n: usize,
    m: usize
}

impl FromStr for Grid {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = Vec::from_iter(s.lines().map(|line| {
            Vec::from_iter(line.chars().map(|c| c.to_digit(10).expect("File contains non-digit chars") as u8))
        }));
        let n = values.len();
        let m = values[0].len();
        Ok( Grid {values, n, m} )
    }
}

type Coordinate = (usize, usize);

struct Ray {
    blocking_tree: Option<Coordinate>,
    dist: usize
}

impl Grid {
    fn cast_ray(&self, (i, j): Coordinate, dir: CompassDirection) -> Ray {
        let mut dist: usize = 0;
        let is_higher = |&(k,l):&Coordinate| {
            dist+=1;
            self.values[k][l]>=self.values[i][j]
        };
        let blocking_tree = match dir {
            North => iproduct!(  (0..i).rev(),   j..=j       ).find(is_higher),
            South => iproduct!( i+1..self.n  ,   j..=j       ).find(is_higher),
            East  => iproduct!(   i..=i      , j+1..self.m   ).find(is_higher),
            West  => iproduct!(   i..=i      ,  (0..j).rev() ).find(is_higher)
        };
        Ray {blocking_tree, dist}
    }

    fn visible_from(&self, pos: Coordinate, dir: CompassDirection) -> bool {
        self.cast_ray(pos, dir).blocking_tree.is_none()
    }

    fn visible(&self, pos: (usize, usize)) -> bool {
        CompassDirection::iter().find(|dir| self.visible_from(pos, *dir)).is_some()
    }

    // part 1 sol
    fn num_visible(&self) -> usize {
        iproduct!(0..self.n, 0..self.m).filter(|&pos| self.visible(pos)).count()
    }

    fn view_score(&self, pos: Coordinate) -> usize {
        CompassDirection::iter().map(|dir| self.cast_ray(pos, dir).dist).product()
    }

    // part 2 sol
    fn best_view(&self) -> usize {
        iproduct!(0..self.n, 0..self.m).map(|pos| self.view_score(pos)).max().unwrap()
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

        let expected = [true, false, false, true];
        for (i, dir) in CompassDirection::iter().enumerate() {
            assert_eq!(grid.visible_from((1,1), dir), expected[i]);
        }

        assert_eq!(grid.num_visible(), 21);
    }
}

fn main() {
    // let contents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let grid: Grid = contents.parse().unwrap();

    // for i in 0..grid.n {
    //     for j in 0..grid.m {
    //         print!("{}: {}  ", grid.values[i][j], if grid.pos_not_hidden((i,j)) {"T"} else {"F"});
    //     }
    //     println!();
    // }

    // let pos = (3,2);
    // for dir in CompassDirection::iter() {
    //     let Ray {blocking_tree, dist} = grid.cast_ray(pos, dir);
    //     println!("{:?} {:?} {:?}", dir, blocking_tree, dist);
    // }
    // dbg!(grid.view_score(pos));

    println!("Num visible: {}", grid.num_visible());
    println!("Num visible: {}", grid.best_view());

}
