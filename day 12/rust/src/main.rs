use std::{fmt::Display, fs};

#[derive(Debug)]
enum Space {
    Start,
    End,
    Mid(char),
}

impl Space {
    fn elevation(&self) -> u8 {
        match *self {
            Space::Start => b'a',
            Space::End => b'z',
            Space::Mid(c) => c as u8,
        }
    }

    fn can_reach(&self, dest: &Space) -> bool {
        dest.elevation() <= self.elevation() + 1
    }
}

type Pos = (usize, usize);
type Row = Vec<Space>;

struct Grid {
    spaces: Vec<Row>,
    height: usize,
    width: usize,
    start: Pos,
}

#[derive(Clone)]
struct Path {
    spaces: Vec<Pos>,
    length: usize,
}

impl Path {
    fn new(start: Pos) -> Self {
        let mut spaces = Vec::with_capacity(100);
        spaces.push(start);
        Path { spaces, length: 1 }
    }

    fn contains(&self, p: &Pos) -> bool {
        self.spaces.contains(&p)
    }

    fn find(&self, pos: &Pos) -> Option<usize> {
        self.spaces.iter().position(|p| p == pos)
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for (i, j) in &self.spaces {
            string += format!("({}, {})  ", i, j).as_str();
        }
        write!(f, "{}", string)?;
        Ok(())
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn dir((fi, fj): Pos, (ti, tj): Pos) -> Direction {
    if ti == fi + 1 {
        Direction::Down
    } else if ti == fi - 1 {
        Direction::Up
    } else if tj == fj + 1 {
        Direction::Right
    } else if tj == fj - 1 {
        Direction::Left
    } else {
        panic!();
    }
}

fn char_dir(from: Pos, to: Pos) -> char {
    match dir(from, to) {
        Direction::Left => '<',
        Direction::Right => '>',
        Direction::Up => '^',
        Direction::Down => 'v',
    }
}

impl Grid {
    fn parse(contents: String) -> Self {
        let mut spaces: Vec<Row> = vec![];
        let mut start: Pos = (0, 0);
        for (i, raw_row) in contents.lines().enumerate() {
            let mut row: Row = vec![];
            for (j, c) in raw_row.chars().enumerate() {
                if c == 'S' {
                    row.push(Space::Start);
                    start = (i, j);
                } else if c == 'E' {
                    row.push(Space::End);
                } else {
                    row.push(Space::Mid(c));
                }
            }
            spaces.push(row);
        }

        let height = spaces.len();
        let width = spaces[0].len();

        Grid {
            spaces,
            height,
            width,
            start,
        }
    }

    fn extend(&self, path: &Path, dir: Direction, max_len: usize) -> Option<Path> {
        let mut new_path = path.clone();
        let (endi, endj) = *path.spaces.last().expect("Path should not be empty");

        let opt_pos = match dir {
            Direction::Left => {
                if endi > 0 {
                    Some((endi - 1, endj))
                } else {
                    None
                }
            }
            Direction::Right => {
                if endi < self.height - 1 {
                    Some((endi + 1, endj))
                } else {
                    None
                }
            }
            Direction::Up => {
                if endj > 0 {
                    Some((endi, endj - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if endj < self.width - 1 {
                    Some((endi, endj + 1))
                } else {
                    None
                }
            }
        };

        if let Some((i, j)) = opt_pos {
            if self.spaces[endi][endj].can_reach(&self.spaces[i][j])
                && !new_path.contains(&(i, j))
                && new_path.length < max_len
            {
                new_path.spaces.push((i, j));
                new_path.length += 1;
                return Some(new_path);
            }
        }
        None
    }

    fn search(&self, max_iter: usize, max_path_size: usize) -> Option<Path> {
        let mut paths: Vec<Path> = Vec::with_capacity(100);
        paths.push(Path::new(self.start));
        let mut shortest: Option<Path> = None;
        let mut max_size: usize = max_path_size;

        let mut iters = 0;
        while let Some(path) = paths.pop() {
            if iters >= max_iter {
                break;
            }
            iters += 1;

            for dir in [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ] {
                if let Some(new_path) = self.extend(&path, dir, max_size) {
                    // println!("{}", new_path);
                    let (i, j) = new_path.spaces.last().unwrap().clone();
                    if let Space::End = self.spaces[i][j] {
                        let sp = match shortest {
                            None => new_path.clone(),
                            Some(sp) => {
                                if sp.length > new_path.length {
                                    new_path.clone()
                                } else {
                                    sp
                                }
                            }
                        };
                        max_size = sp.length;
                        shortest = Some(sp);
                    } else {
                        paths.push(new_path);
                    }
                }
            }

            if iters % 1_000_000 == 0 {
                dbg!(paths.len());
            }
        }

        if iters == max_iter {
            self.vis_path(&paths.last().unwrap());
            self.path_elevation(&paths.last().unwrap());
        }
        dbg!(iters);
        shortest
    }

    fn vis_path(&self, path: &Path) {
        for i in 0..self.height {
            for j in 0..self.width {
                let mut c = '.';
                if let Some(k) = path.find(&(i, j)) {
                    if k < path.length - 1 {
                        c = char_dir(path.spaces[k], path.spaces[k + 1]);
                    }
                }
                print!("{}", c);
            }
            println!("");
        }
    }

    fn path_elevation(&self, path: &Path) {
        for i in 0..self.height {
            for j in 0..self.width {
                let mut c = '.';
                if path.contains(&(i, j)) {
                    c = self.spaces[i][j].elevation() as char;
                }
                print!("{}", c);
            }
            println!("");
        }
    }
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    // let contents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let grid = Grid::parse(contents);
    println!("{}", grid.height);
    println!("{}", grid.width);

    if let Some(min_path) = grid.search(10_000_000, 500) {
        println!("{}", min_path);
        println!("{}", min_path.length - 1);

        grid.vis_path(&min_path);
    }
}
