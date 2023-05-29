use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split(",");
        match (sp.next(), sp.next()) {
            (Some(xs), Some(ys)) => {
                let x: i32 = xs.parse().expect("x is not an i32");
                let y: i32 = ys.parse().expect("y is not an i32");
                Ok(Point { x, y })
            }
            _ => Err("Expected \"x,y\" input".to_string()),
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    /// bounding box bottom left
    fn bb_tl(&self, other: Point) -> Point {
        Point {
            x: min(self.x, other.x),
            y: min(self.y, other.y),
        }
    }
    /// bounding box top right
    fn bb_br(&self, other: Point) -> Point {
        Point {
            x: max(self.x, other.x),
            y: max(self.y, other.y),
        }
    }
}

#[derive(Debug)]
struct Path {
    points: Vec<Point>,
}

impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Point> = s
            .split(" -> ")
            .map(|ps| ps.parse().expect("Invalid point in path string"))
            .collect();
        Ok(Path { points })
    }
}

impl From<Vec<Point>> for Path {
    fn from(points: Vec<Point>) -> Self {
        Path { points }
    }
}

struct LineSegmentIterator {
    increment_x: bool,
    coord_max: i32,
    current: Point,
}

impl Iterator for LineSegmentIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.current; // inclusive, so endpoints will overlap
        let cont = match self.increment_x {
            true => {
                self.current.x += 1;
                self.current.x <= self.coord_max + 1
            }
            false => {
                self.current.y += 1;
                self.current.y <= self.coord_max + 1
            }
        };

        cont.then(|| curr)
    }
}

impl LineSegmentIterator {
    fn new(start: Point, end: Point) -> Self {
        let increment_x = start.x != end.x;
        let (coord_max, current) = match increment_x {
            true => match start.x <= end.x {
                true => (end.x, start),
                false => (start.x, end),
            },
            false => match start.y <= end.y {
                true => (end.y, start),
                false => (start.y, end),
            },
        };
        LineSegmentIterator {
            increment_x,
            coord_max,
            current,
        }
    }
}

impl Path {
    /// bounding box bottom left
    fn bb_tl(&self, extra: Point) -> Point {
        self.points.iter().fold(extra, |acc, p| acc.bb_tl(*p))
    }
    /// bounding box top right
    fn bb_br(&self, extra: Point) -> Point {
        self.points.iter().fold(extra, |acc, p| acc.bb_br(*p))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GridSpace {
    Rock,
    Air,
    Sand,
}

struct Grid {
    spaces: HashMap<Point, GridSpace>,
    /// top left
    tl: Point,
    /// bottom right
    br: Point,
    height: i32,
    width: i32,
}

impl Grid {
    fn from_walls(walls: Vec<Path>) -> Grid {
        let source = Point { x: 500, y: 0 };
        let tl = Path::from(
            walls
                .iter()
                .map(|wall| wall.bb_tl(source))
                .collect::<Vec<Point>>(),
        )
        .bb_tl(source);
        let br = Path::from(
            walls
                .iter()
                .map(|wall| wall.bb_br(source))
                .collect::<Vec<Point>>(),
        )
        .bb_br(source);

        let height = 1 + br.x - tl.x;
        let width = 1 + br.y - tl.y;

        let mut spaces: HashMap<Point, GridSpace> = HashMap::new();

        for wall in walls {
            for i in 0..wall.points.len() - 1 {
                // overlapping endpoints set twice for brevity
                for point in LineSegmentIterator::new(wall.points[i], wall.points[i + 1]) {
                    spaces.entry(point).or_insert(GridSpace::Rock);
                }
            }
        }

        Grid {
            spaces,
            height,
            width,
            tl,
            br,
        }
    }

    fn is_in_bounds(&self, pos: Point) -> bool {
        self.tl.x <= pos.x && pos.x < self.br.x && self.tl.y <= pos.y && pos.y < self.br.y
    }

    fn available(&self, pos: Point) -> bool {
        if let Some(gs) = self.spaces.get(&pos) {
            return *gs == GridSpace::Air;
        }
        true
    }

    fn next_pos(&self, previous_pos: Point) -> Option<Point> {
        let down = Point::new(previous_pos.x, previous_pos.y + 1);
        if self.available(down) {
            return Some(down);
        }
        let left = Point::new(previous_pos.x - 1, previous_pos.y + 1);
        if self.available(left) {
            return Some(left);
        }
        let right = Point::new(previous_pos.x + 1, previous_pos.y + 1);
        if self.available(right) {
            return Some(right);
        }
        None
    }

    fn drop_grain(&mut self) -> bool {
        let mut grain_pos = Point { x: 500, y: 0 };

        while let Some(new_pos) = self.next_pos(grain_pos) {
            grain_pos = new_pos;
            if !self.is_in_bounds(grain_pos) {
                return false;
            }
        }

        // dbg!(grain_pos);

        // not cheching for original position being occupied
        self.spaces.entry(grain_pos).or_insert(GridSpace::Sand);
        true
    }

    fn vis(&self) {
        for y in self.tl.y..=self.br.y {
            for x in self.tl.x..=self.br.x {
                if let Some(gs) = self.spaces.get(&Point { x, y }) {
                    match gs {
                        GridSpace::Air => print!("."),
                        GridSpace::Rock => print!("#"),
                        GridSpace::Sand => print!("o"),
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn part_1(&mut self) {
        let mut i = 0;
        while self.drop_grain() {
            i += 1;
        }
        self.vis();
        dbg!(i);
    }
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    // let contents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let walls: Vec<Path> = contents.lines().map(|line| line.parse().unwrap()).collect();
    let mut grid = Grid::from_walls(walls);

    grid.part_1();
}
