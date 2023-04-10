
#[derive(Debug)]
struct File {
    name: String,
    size: usize
}

#[derive(Debug)]
struct Dir {
    name: String,
    parent: Option<Box<Dir>>,
    children: Vec<Box<FSNode>>
}

#[derive(Debug)]
enum FSNode {
    File(File),
    Dir(Dir)
}

#[derive(Debug)]
struct DirTree {
    root: FSNode,
    current_dir: Box<FSNode>,
}

trait Size {
    fn get_size(&self) -> usize;
}

impl Size for File {
    fn get_size(&self) -> usize {
        self.size
    }
}

impl Size for Dir {
    fn get_size(&self) -> usize {
        self.children.iter().map(|c| {
            match c.as_ref() {
                FSNode::File(file) => file.get_size(),
                FSNode::Dir(dir) => dir.get_size(),
            }
        }).sum()
    }
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

    fn subdirs(&self) -> Vec<&Box<FSNode>> {
        self.children.iter().filter(|c| {
            if let FSNode::Dir(dir) = c.as_ref() {true} else {false}
        }).collect()
    }
}

fn main() {
    let mut root = Dir {name:"/".to_string(), parent:None, children:vec![]};
    root.add_file(File {name:"a".to_string(), size:50});
    root.add_dir(Dir {name:"b".to_string(), parent:None, children:vec![]});

    root.ls();

    for c in root.subdirs() {
        dbg!(c);
    }
}
