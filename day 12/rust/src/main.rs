use std::{borrow::Borrow, cell::RefCell, fmt::Display, fs, rc::Rc};

#[derive(Debug)]
enum Elevation {
    Start,
    End,
    Mid(u8),
}

type SpaceRef = Rc<RefCell<Space>>;
struct Space {
    elevation: Elevation,
    visited: bool,
    left: Option<SpaceRef>,
    right: Option<SpaceRef>,
    above: Option<SpaceRef>,
    below: Option<SpaceRef>,
}

// Space option to string
fn so2s(opt: &Option<SpaceRef>) -> String {
    if let Some(sr) = &*opt {
        format!("{:?}", (**sr).borrow().elevation)
    } else {
        "None".to_string()
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "dat:{:?} left:{:?} right:{:?} bottom:{:?} top:{:?}",
            self.elevation,
            so2s(&self.left),
            so2s(&self.right),
            so2s(&self.below),
            so2s(&self.above)
        )
    }
}

impl Space {
    fn new(elevation: Elevation) -> SpaceRef {
        Rc::new(RefCell::new(Space {
            elevation,
            visited: false,
            left: None,
            right: None,
            above: None,
            below: None,
        }))
    }
}

type Row = Vec<SpaceRef>;
struct Grid {
    spaces: Vec<Row>,
    height: usize,
    width: usize,
}

impl Grid {
    fn parse(contents: String) -> Self {
        let mut spaces: Vec<Row> = vec![];
        for raw_row in contents.lines() {
            let mut row: Row = vec![];
            for c in raw_row.chars() {
                let i = c as u8;
                if (97..123).contains(&i) {
                    row.push(Space::new(Elevation::Mid(i)));
                } else if i == 83 {
                    row.push(Space::new(Elevation::Start));
                } else if i == 69 {
                    row.push(Space::new(Elevation::End));
                } else {
                    panic!("Unexpected char value in file.");
                }
            }
            spaces.push(row);
        }

        let height = spaces.len();
        let width = spaces[0].len();
        for i in 0..height {
            for j in 0..width {
                if i > 0 {
                    spaces[i][j].borrow_mut().left = Some(spaces[i - 1][j].clone());
                }
                if i < height - 1 {
                    spaces[i][j].borrow_mut().right = Some(spaces[i + 1][j].clone());
                }
                if j > 0 {
                    spaces[i][j].borrow_mut().above = Some(spaces[i][j - 1].clone());
                }
                if j < width - 1 {
                    spaces[i][j].borrow_mut().below = Some(spaces[i][j + 1].clone());
                }
            }
        }

        Grid {
            spaces,
            height,
            width,
        }
    }
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone)]
struct Path {
    visited_spaces: Vec<SpaceRef>,
    directions: Vec<Direction>,
}

impl Path {
    fn extend_left(&self) -> Result<Self, ()> {
        let end = &**self.visited_spaces.last().unwrap();
        if let Some(sr) = end.borrow().left.borrow() {
            let mut new_path = self.clone();
            new_path.visited_spaces.push(sr.clone());
        }
        Err(())
    }
}

static TEST_INPUT_PATH: &str = "../test_input";
static INPUT_PATH: &str = "../input";

fn main() {
    let contents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    // let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let grid = Grid::parse(contents);

    println!("{}", (*grid.spaces[0][0]).borrow());
    println!("{}", (*grid.spaces[1][1]).borrow());
}
