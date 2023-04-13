use std::{fmt::Debug, fs::File};

// type RcRc<T> = Rc<RefCell<T>>;

#[derive(Debug)]
enum NodeVariant<T: Debug> {
    Inner {children: Vec<usize>},
    Leaf {data: T}
}

#[derive(Debug)]
struct Node<T: Debug> {
    idx: usize, // will change if nodes removed, not needed if not backtracking?
    parent: Option<usize>,
    name: String,
    variant: NodeVariant<T>
}

trait NamedVariants { // does not pick up default impl?
    fn inner_name() -> String { "Inner".to_string() }
    fn leaf_name() -> String { "Leaf".to_string() }
}

impl<T: Debug> ToString for Node<T> where Node<T> : NamedVariants {
    fn to_string(&self) -> String {
        match &self.variant {
            NodeVariant::Inner { children:_ } => format!("{} ({})", Node::<T>::inner_name(), self.name),
            NodeVariant::Leaf { data } => format!("{} ({}, {:?})", Node::<T>::leaf_name(), self.name, data)
        }
    }
}

#[derive(Debug)]
struct FlatTree<T: Debug> {
    nodes: Vec<Node<T>>
}

impl<T: Debug> FlatTree<T> {
    fn new() -> FlatTree<T> {
        FlatTree { nodes: vec![] }
    }

    fn new_node(&mut self, name: String, 
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

}

impl<T: Debug> FlatTree<T> where Node<T> : NamedVariants {
    fn ls(&self, idx: usize) {
        if let NodeVariant::Inner { children } = &self.nodes[idx].variant {
            println!("{}", &self.nodes[idx].name);
            for &i in children {
                println!("  {}", &self.nodes[i].to_string());
            }
        }
    }
}

#[derive(Debug)]
struct FileData(usize);

impl NamedVariants for Node<FileData> {
    fn inner_name() -> String { "Dir".to_string() }
    fn leaf_name() -> String { "File".to_string() }
}

impl FlatTree<FileData> {
    fn subdirs(&self, idx: usize) -> Vec<usize> {
        if let NodeVariant::Inner { children } = &self.nodes[idx].variant {
            let mut out = vec![];
            for &c in children {
                if let NodeVariant::Inner { children:_ } = &self.nodes[c].variant {
                    out.push(c);
                }
            } // had to trouble with filter and collecting &usize
            out
        } else {
            vec![]
        }
    }
}

fn main() {
    let mut tree = FlatTree::<FileData>::new();

    tree.new_node("/".to_string(), None, None);

    tree.new_node("a".to_string(), Some(FileData(50)), Some(0));
    tree.new_node("b".to_string(), None, Some(0));
    tree.new_node("c".to_string(), Some(FileData(25)), Some(0));

    tree.new_node("d".to_string(), Some(FileData(25)), Some(2));
    tree.new_node("e".to_string(), Some(FileData(25)), Some(2));
    tree.new_node("f".to_string(), None, Some(0));

    tree.ls(0);
    tree.ls(2);

    dbg!(tree.subdirs(0));

}
