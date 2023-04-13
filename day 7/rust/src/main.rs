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
            true => format!("{} ({})", self.name, "dir"),
            
        }
    }
}

impl FlatTree<FileData> {
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

fn build_tree(contents: &String, tree: &mut FlatTree<FileData>) {
    for line in contents.lines() {
        match line.parse::<Input>().unwrap() {
            Input::ListedDir { name } => tree.new_here(name, FileData {size: None, is_dir: true}),
            Input::ListedFile { name, size } => tree.new_here(name, FileData {size: Some(size), is_dir: false}),
            Input::Cd { to_dir } => {
                if to_dir=="/" {
                    tree.to_root()
                } else if to_dir==".." {
                    tree.traverse_up()
                } else {
                    tree.traverse_into(to_dir)
                }
            },
            _ => ()
        }
    }
}

static INPUT_PATH : &str = "../input";
static TEST_INPUT_PATH : &str = "../test_input";

fn main() {
    let mut tree = FlatTree::<FileData>::new(); // should initialise with root?
    tree.new_node("/".to_string(), FileData {size: None, is_dir: true}, None);

    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");

    build_tree(&tcontents, &mut tree);

    tree.ls(0);
    tree.ls(4);

}
