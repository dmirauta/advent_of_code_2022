use std::fmt::Debug;

// type RcRc<T> = Rc<RefCell<T>>;

#[derive(Debug)]
pub enum NodeVariant<T: Debug> {
    Inner {children: Vec<usize>},
    Leaf {data: T}
}

#[derive(Debug)]
pub struct Node<T: Debug> {
    pub idx: usize, // will change if nodes removed, not needed if not backtracking?
    pub parent: Option<usize>,
    pub name: String,
    pub variant: NodeVariant<T>
}

#[derive(Debug)]
pub struct FlatTree<T: Debug> {
    pub nodes: Vec<Node<T>>,
    pub current: Option<usize> // for traversal, should only point at Inner variant, but not constrained
}

impl<T: Debug> FlatTree<T> {
    pub fn new() -> FlatTree<T> {
        FlatTree { nodes: vec![], current:None }
    }

    pub fn new_node(&mut self, name: String, 
                           data: Option<T>, 
                           parent: Option<usize>) {
        let variant = match data {
            Some(d) => NodeVariant::Leaf { data: d },
            None       => NodeVariant::Inner { children: vec![] }
        };
        let idx = self.nodes.len();
        if let Some(pidx) = parent {
            if let NodeVariant::Inner {children} = &mut self.nodes[pidx].variant {
                children.push(idx);
            }
        }
        self.nodes.push( Node {idx, parent, name, variant} );
    }

    pub fn new_here(&mut self, name: String, data: Option<T>, ) {
        self.new_node(name, data, self.current)
    }

    pub fn traverse_into(&mut self, name:String) {
        if let Some(i) = self.current {
            if let NodeVariant::Inner { children } = &self.nodes[i].variant {
                for &ci in children {
                    if self.nodes[ci].name == name { // should check if innner here really
                        self.current = Some(ci);
                        break
                    }
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
    pub fn print_children(&self, idx: usize) {
        if let NodeVariant::Inner { children } = &self.nodes[idx].variant {
            println!("{}", &self.nodes[idx].name);
            for &i in children {
                println!("  {}", &self.nodes[i].to_string());
            }
        }
    }
}