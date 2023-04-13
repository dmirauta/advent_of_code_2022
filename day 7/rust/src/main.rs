use std::fmt::Debug;

mod tree;
use tree::*;

#[derive(Debug)]
struct FileData(usize);

impl NamedVariants for Node<FileData> {
    fn inner_name() -> String { "Dir".to_string() }
    fn leaf_name() -> String { "File".to_string() }
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
}

fn main() {
    let mut tree = FlatTree::<FileData>::new();

    tree.new_node("/".to_string(), None, None);
    tree.to_root();

    tree.new_here("a".to_string(), Some(FileData(50)));
    tree.new_here("b".to_string(), None);
    tree.new_here("c".to_string(), Some(FileData(25)));

    tree.traverse_into("b".to_string());
    tree.new_here("d".to_string(), Some(FileData(25)));
    tree.new_here("e".to_string(), Some(FileData(25)));

    tree.to_root();
    tree.new_here("f".to_string(), None);

    tree.ls(0);
    tree.ls(2);

    dbg!(tree.subdirs(0));

}
