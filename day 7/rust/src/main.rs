use std::{fmt::Debug};

// type RcRc<T> = Rc<RefCell<T>>;

#[derive(Debug)]
enum Node<T: Debug> {
    Inner {children: Vec<usize>},
    Leaf {data: T}
}

#[derive(Debug)]
struct NodeWrap<T: Debug> {
    idx: usize, // will change if nodes removed, not needed if not backtracking?
    name: String,
    node: Node<T>
}

// Naming specific to dirtree...
impl<T: Debug> ToString for NodeWrap<T> {
    fn to_string(&self) -> String {
        match &self.node {
            Node::Inner { children:_ } => format!("{} (Dir)", self.name),
            Node::Leaf { data } => format!("{} (File, {:?})", self.name, data)
        }
    }
}

#[derive(Debug)]
struct FlatTree<T: Debug> {
    nodes: Vec<NodeWrap<T>>
}

impl<T: Debug> FlatTree<T> {
    fn new() -> FlatTree<T> {
        FlatTree { nodes: vec![] }
    }

    fn new_node(&mut self, name: String, 
                           data: Option<T>, 
                           parent: Option<usize>) {
        let node = if let Some(d) = data {
            Node::Leaf { data: d }
        } else {
            Node::Inner { children: vec![] }
        };
        let idx = self.nodes.len();
        if let Some(pidx) = parent {
            if let Node::Inner {children} = &mut self.nodes[pidx].node {
                children.push(idx);
            }
        }
        let nw = NodeWrap {idx, name, node};
        self.nodes.push(nw);
    }

    fn ls(&self, idx: usize) {
        if let Node::Inner { children } = &self.nodes[idx].node {
            for &i in children {
                println!("{}", &self.nodes[i].to_string());
            }
        }
    }

    fn subdirs(&self, idx: usize) -> Vec<usize> {
        if let Node::Inner { children } = &self.nodes[idx].node {
            let mut out = vec![];
            for &c in children {
                if let Node::Inner { children } = &self.nodes[c].node {
                    out.push(c);
                }
            } // had to trouble with filter and collecting &usize
            out
        } else {
            vec![]
        }
    }
}

#[derive(Debug)]
struct FileData(usize);

fn main() {
    let mut tree = FlatTree::<FileData>::new();

    tree.new_node("/".to_string(), None, None);
    tree.new_node("a".to_string(), Some(FileData(50)), Some(0));
    tree.new_node("b".to_string(), None, Some(0));

    tree.ls(0);

    dbg!(tree.subdirs(0));
    dbg!(tree.subdirs(0));
}
