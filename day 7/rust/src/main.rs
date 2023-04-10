
struct File {
    name: String,
    size: usize
}

struct Dir {
    name: String,
    parent: Option<Box<Dir>>,
    children: Vec<Box<FSNode>>
}

impl Dir {
    fn add_file(&mut self, child: File) {
        self.children.push( Box::new(FSNode::File(child)) );
    }
    fn add_dir(&mut self, child: Dir) {
        self.children.push( Box::new(FSNode::Dir(child)) );
    }
    fn ls(&self) {
        println!("ls {}", self.name);
        for fsn in self.children.iter() {
            match fsn.as_ref() {
                FSNode::File(file) => println!("{} {}", file.name, file.size),
                FSNode::Dir(dir) => println!("{} {}", dir.name, dir.children.len()),
            }
        }
        println!("");
    }
}

enum FSNode {
    File(File),
    Dir(Dir)
}

struct DirTree {
    root: FSNode,
    current_dir: Box<FSNode>,
}

fn main() {
    let mut root = Dir {name:"/".to_string(), parent:None, children:vec![]};
    root.add_file(File {name:"a".to_string(), size:50});
    root.add_file(File {name:"b".to_string(), size:60});

    root.ls();
}
