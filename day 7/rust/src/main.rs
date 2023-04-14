use std::{fmt::Debug, str::FromStr, fs};
use regex::Regex;

#[macro_use]
extern crate lazy_static;

mod tree;
use tree::*;

#[derive(Debug)]
struct FileData {
    size: Option<usize>,
    is_dir: bool
}

impl ToString for Node<FileData> {
    fn to_string(&self) -> String {
        match self.data.is_dir {
            false => format!("{} ({}, size={:?})", self.name, "file", self.data.size.unwrap()),
            true  => match self.data.size {
                Some(s) => format!("{} ({}, total size={:?})", self.name, "dir", s),
                None => format!("{} ({})", self.name, "dir"),
            }
        }
    }
}

#[derive(Debug)]
enum Input {
    Cd{to_dir:String},
    Ls,
    ListedDir{name: String},
    ListedFile{name: String, size: usize}
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_CD: Regex   = Regex::new(r"\$ cd (.*)").unwrap();
            static ref RE_DIR: Regex  = Regex::new(r"dir (.*)").unwrap();
            static ref RE_FILE: Regex = Regex::new(r"(\d*) (.*)").unwrap();
        }

        if s==r"$ ls" {
            return Ok( Input::Ls );
        } else if let Some(cap) = RE_CD.captures(s) {
            return Ok( Input::Cd { to_dir: cap.get(1).unwrap().as_str().to_string() } );
        } else if let Some(cap) = RE_DIR.captures(s) {
            return Ok( Input::ListedDir { name: cap.get(1).unwrap().as_str().to_string() } );
        } else if let Some(cap) = RE_FILE.captures(s) {
            return Ok( Input::ListedFile { name: cap.get(2).unwrap().as_str().to_string(), 
                                           size: cap.get(1).unwrap().as_str().parse().unwrap() } );
        }
        Err( "Line had no match".to_string() )
    }
}

impl FlatTree<FileData> {
    fn _ls(&self, idx: usize) {
        self._print_children(idx, "  ".to_string(), 0, 3);
    }

    fn parse_lines(&mut self, contents: &String) {
        for line in contents.lines() {
            match line.parse::<Input>().unwrap() {
                Input::ListedDir { name } => self.new_here(name, FileData {size: None, is_dir: true}),
                Input::ListedFile { name, size } => self.new_here(name, FileData {size: Some(size), is_dir: false}),
                Input::Cd { to_dir } => {
                    if to_dir=="/" {
                        self.to_root()
                    } else if to_dir==".." {
                        self.traverse_up()
                    } else {
                        self.traverse_into(to_dir)
                    }
                },
                _ => ()
            }
        }
    }

    fn try_calc_size(&mut self, idx: usize) {
        if let Some(_) = self.nodes[idx].data.size {
            return; // bail if already calculated
        }
        let mut sum = 0;
        let mut failed_at: Vec<usize> = Vec::with_capacity(INIT_CHILD_NODE_CAP);
        for ci in self.nodes[idx].children.iter() {
            if let None = self.nodes[*ci].data.size {
                failed_at.push(*ci);
            } else {
                sum += self.nodes[*ci].data.size.unwrap();
            }
        }
        if failed_at.len()>0 {
            for ci in failed_at {
                self.try_calc_size(ci);
            }
            self.try_calc_size(idx); // try again, could probably keep working on partial sum?
        } else {
            self.nodes[idx].data.size = Some(sum);
        }
    }

    fn from_file(contents: &String) -> FlatTree<FileData> {
        let n = contents.lines().count();
    
        let mut tree = FlatTree::<FileData>::new(n); // should initialise with root?
        tree.new_node("/".to_string(), FileData {size: None, is_dir: true}, None);
        tree.parse_lines(&contents);
    
        // calc dir sizes
        let n = tree.nodes.len();
        for idx in (0..n).rev() { // calc sizes for (heuristically) outermost dirs first
            tree.try_calc_size(idx);
        }
    
        return tree;
    }

}

fn part_1(tree: &FlatTree<FileData>) {
    let ans:usize = tree.nodes.iter().filter(|n| n.data.is_dir)
                                     .map(|n| n.data.size.unwrap())
                                     .filter(|s| *s<=(100_000 as usize)).sum();
    println!("Sum of at most 100000: {}", ans);
}

static TOTAL_CAPACITY:usize = 70000000;
static NEEDED_FREE:usize    = 30000000;

fn part_2(tree: &FlatTree<FileData>) {
    let REQ_CAP = TOTAL_CAPACITY - NEEDED_FREE;
    let MIN_TO_FREE = tree.nodes[0].data.size.unwrap() - REQ_CAP;
    let ans = tree.nodes.iter().filter(|n| n.data.is_dir && n.data.size.unwrap()>MIN_TO_FREE)
                                                        .min_by_key(|n| n.data.size.unwrap()).unwrap();
    println!("Smallest directory {} to create desired space, with total size {}", ans.name, ans.data.size.unwrap());
}

static INPUT_PATH : &str = "../input";
static TEST_INPUT_PATH : &str = "../test_input";

fn main() {
    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    let ttree = FlatTree::<FileData>::from_file(&tcontents);
    let tree = FlatTree::<FileData>::from_file(&contents);

    // ttree._ls(0);

    part_1(&ttree);
    part_1(&tree);

    part_2(&ttree);
    part_2(&tree);

}
