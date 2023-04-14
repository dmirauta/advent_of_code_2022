use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T: Debug> {
    pub idx: usize, // will change if nodes removed, not needed if not backtracking?
    pub parent: Option<usize>,
    pub name: String,
    pub data: T,
    pub children: Vec<usize>
}

#[derive(Debug)]
pub struct FlatTree<T: Debug> {
    pub nodes: Vec<Node<T>>,
    pub current: Option<usize>
}

impl<T: Debug> FlatTree<T> {
    pub fn new() -> FlatTree<T> {
        FlatTree { nodes: vec![], current:None }
    }

    pub fn new_node(&mut self, name: String, 
                           data: T, 
                           parent: Option<usize>) {
        let idx = self.nodes.len();
        if let Some(pidx) = parent {
            self.nodes[pidx].children.push(idx)
        }
        let children: Vec<usize> = vec![];
        self.nodes.push( Node {idx, parent, name, data, children} );
    }

    pub fn new_here(&mut self, name: String, data: T, ) {
        self.new_node(name, data, self.current)
    }

    pub fn traverse_into(&mut self, name:String) {
        if let Some(i) = self.current {
            for &ci in &self.nodes[i].children {
                if self.nodes[ci].name == name {
                    self.current = Some(ci);
                    break
                }
            }
        }
    }

    pub fn traverse_up(&mut self) {
        if let Some(i) = self.current {
            if let Some(pi) = self.nodes[i].parent {
                self.current = Some(pi)
            }
        }
    }

    pub fn to_root(&mut self) {
        if self.nodes.len()>0 {
            self.current = Some(0)
        }
    }
}

impl<T: Debug> FlatTree<T> where Node<T> : ToString {
    pub fn _print_children(&self, idx: usize) {
        println!("{}", &self.nodes[idx].name);
        for &i in &self.nodes[idx].children {
            println!("  {}", &self.nodes[i].to_string());
        }
    }
}