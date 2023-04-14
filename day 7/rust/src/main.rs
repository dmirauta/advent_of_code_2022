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
            true  => format!("{} ({})", self.name, "dir"),
            
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
    fn ls(&self, idx: usize) {
        self.print_children(idx);
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
        let mut sum = 0;
        let mut failed_at: Vec<usize> = vec![];
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

}

fn part_1(contents: &String) {
    let mut tree = FlatTree::<FileData>::new(); // should initialise with root?
    tree.new_node("/".to_string(), FileData {size: None, is_dir: true}, None);
    tree.parse_lines(&contents);

    tree.try_calc_size(0);

    let max_size: usize = 100_000;
    // for n in tree.nodes.iter().filter(|n| n.data.is_dir && n.data.size.unwrap()<max_size) {
    //     println!("{} {}", n.name, n.data.size.unwrap());
    // }
    
    println!("Sum of at most 100000: {}", tree.nodes.iter()
                                            .filter(|n| n.data.is_dir)
                                            .map(|n| n.data.size.unwrap())
                                            .filter(|&s| s<max_size).sum::<usize>());
}

static INPUT_PATH : &str = "../input";
static TEST_INPUT_PATH : &str = "../test_input";

fn main() {
    let tcontents = fs::read_to_string(TEST_INPUT_PATH).expect("Could not read {TEST_INPUT_PATH}");
    let contents = fs::read_to_string(INPUT_PATH).expect("Could not read {INPUT_PATH}");

    part_1(&tcontents);
    part_1(&contents);

}
