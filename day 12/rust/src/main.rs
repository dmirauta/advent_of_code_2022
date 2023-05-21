use std::{collections::VecDeque, fmt::Display, fs};

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
#[derive(Clone)]
struct ApproachDescriptor {
    pos: Option<Pos>,
    dist: usize,
}

impl ApproachDescriptor {
    fn new() -> Self {
        ApproachDescriptor { pos: None, dist: 0 }
    }
}

type Row = Vec<Space>;
struct Grid {
    spaces: Vec<Row>,
    best_approach: Vec<Vec<ApproachDescriptor>>,
    height: usize,
    width: usize,
    start: Pos,
    end: Pos,
}

#[derive(Clone)]
struct Path {
    spaces: Vec<Pos>,
}

impl Path {
    fn new(start: Pos) -> Self {
        let mut spaces = Vec::with_capacity(100);
        spaces.push(start);
        Path { spaces }
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
    } else if ti + 1 == fi {
        Direction::Up
    } else if tj == fj + 1 {
        Direction::Right
    } else if tj + 1 == fj {
        Direction::Left
    } else {
        panic!("Unexpected jump");
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
        let mut end: Pos = (0, 0);
        for (i, raw_row) in contents.lines().enumerate() {
            let mut row: Row = vec![];
            for (j, c) in raw_row.chars().enumerate() {
                if c == 'S' {
                    row.push(Space::Start);
                    start = (i, j);
                } else if c == 'E' {
                    row.push(Space::End);
                    end = (i, j);
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
            best_approach: vec![vec![ApproachDescriptor::new(); width]; height],
            height,
            width,
            start,
            end,
        }
    }

    fn floodfill(&mut self) {
        self.best_approach[self.start.0][self.start.1].pos = Some(self.start);
        let mut queue: VecDeque<Pos> = VecDeque::from([self.start]);

        while let Some((i, j)) = queue.pop_front() {
            let from_dist = self.best_approach[i][j].dist;

            for dir in [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ] {
                if let Some((ni, nj)) = self.try_march((i, j), dir) {
                    if let None = self.best_approach[ni][nj].pos {
                        self.best_approach[ni][nj].pos = Some((i, j));
                        self.best_approach[ni][nj].dist = from_dist + 1;
                        queue.push_back((ni, nj));
                    }
                }
            }
        }
    }

    fn try_march(&self, (i, j): Pos, dir: Direction) -> Option<Pos> {
        let opt_pos = match dir {
            Direction::Left => {
                if i > 0 {
                    Some((i - 1, j))
                } else {
                    None
                }
            }
            Direction::Right => {
                if i < self.height - 1 {
                    Some((i + 1, j))
                } else {
                    None
                }
            }
            Direction::Up => {
                if j > 0 {
                    Some((i, j - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if j < self.width - 1 {
                    Some((i, j + 1))
                } else {
                    None
                }
            }
        };

        if let Some((ni, nj)) = opt_pos {
            if self.spaces[i][j].can_reach(&self.spaces[ni][nj]) {
                return Some((ni, nj));
            }
        }
        None
    }

    fn backtrace(&self) -> Path {
        let mut shortest: Path = Path::new(self.end);
        loop {
            let (i, j) = *shortest.spaces.last().unwrap();
            let prev = self.best_approach[i][j].pos.unwrap();
            shortest.spaces.push(prev);
            if prev == self.start {
                break;
            }
        }
        shortest.spaces.reverse();
        shortest
    }

    fn vis_approach(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let pd = self.best_approach[i][j].dist;
                if let Some((pi, pj)) = self.best_approach[i][j].pos {
                    print!("({} {}| {}) ", pi, pj, pd);
                } else {
                    print!("(None| {}) ", pd);
                }
            }
            println!();
        }
    }

    fn vis_path(&self, path: &Path) {
        for i in 0..self.height {
            for j in 0..self.width {
                let mut c = '.';
                if let Some(k) = path.find(&(i, j)) {
                    if k < path.spaces.len() - 1 {
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

    let mut grid = Grid::parse(contents);
    // println!("{}", grid.height);
    // println!("{}", grid.width);

    grid.floodfill();

    // grid.vis_approach();

    let shortest = grid.backtrace();
    grid.vis_path(&shortest);
    dbg!(shortest.spaces.len() - 1);
}
