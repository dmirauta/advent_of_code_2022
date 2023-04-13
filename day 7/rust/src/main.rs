use std::{fmt::Debug, str::FromStr, fs};
use regex::Regex;

#[macro_use]
extern crate lazy_static;

mod tree;
use tree::*;

#[derive(Debug)]
struct FileData(usize);

impl ToString for Node<FileData> {
    fn to_string(&self) -> String {
        match &self.variant {
            NodeVariant::Inner { children:_ } => format!("{} ({})", "Dir", self.name),
            NodeVariant::Leaf { data } => format!("{} ({}, {:?})", "File", self.name, data)
        }
    }
}

impl FlatTree<FileData> {
    fn subdirs(&self, idx: usize) -> Vec<usize> {
        if let NodeVariant::Inner { children } = &self.nodes[idx].variant {
            children.iter().filter(|c| match &self.nodes[**c].variant {
                NodeVariant::Inner {children:_} => true, _ => false
            }).cloned().collect()
        } else {
            vec![]
        }
    }

    fn ls(&self, idx: usize) {
        self.print_children(idx);
    }
}

#[derive(Debug)]
enum Input {
    Cd{to_dir:String},
    Ls,
    ListedDir{name: String},
    ListedFile{name: String, size: usize}
}

static CD_PATTERN: &str = r"\$ cd (.*)";
static DIR_PATTERN: &str = r"dir (.*)";
static FILE_PATTERN: &str = r"(\d*) (.*)";

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_CD: Regex   = Regex::new(CD_PATTERN).unwrap();
            static ref RE_DIR: Regex  = Regex::new(DIR_PATTERN).unwrap();
            static ref RE_FILE: Regex = Regex::new(FILE_PATTERN).unwrap();
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

static INPUT_PATH : &str = "../input";
static TEST_INPUT_PATH : &str = "../test_input";

fn main() {
    let mut tree = FlatTree::<FileData>::new();

    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");

    for line in tcontents.lines() {
        dbg!(line.parse::<Input>().unwrap());
    }

}
