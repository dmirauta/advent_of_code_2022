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

pub trait NamedVariants { // does not pick up default impl?
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
pub struct FlatTree<T: Debug> {
    pub nodes: Vec<Node<T>>
}

impl<T: Debug> FlatTree<T> {
    pub fn new() -> FlatTree<T> {
        FlatTree { nodes: vec![] }
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

}

impl<T: Debug> FlatTree<T> where Node<T> : NamedVariants {
    pub fn ls(&self, idx: usize) {
        if let NodeVariant::Inner { children } = &self.nodes[idx].variant {
            println!("{}", &self.nodes[idx].name);
            for &i in children {
                println!("  {}", &self.nodes[i].to_string());
            }
        }
    }
}