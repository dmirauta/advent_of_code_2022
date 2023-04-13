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
