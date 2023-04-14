import re

with open("input", "r") as f:
    lines = f.readlines()

with open("test_input", "r") as f:
    test_lines = f.readlines()

class Dir:

    def __init__(self, name, parent=None, children=None):
        self.name : str = name
        self.parent : Dir = parent
        self.children : dict[str, Dir | File] = children if not children is None else {}
        self.size = None # buffers size calc

    def subdirs(self):
        return list(filter(lambda c: type(c)==Dir, self.children.values()))

    def get_size(self, recalc=False):
        # also could be done non recursively...
        if recalc or self.size is None:
            self.size = sum([ c.get_size() for c in self.children.values() ])
        return self.size

    def ls(self, prefix="", depth=-1, original_call=True):
        # would probably be nicer depth first, rather than recursive

        if original_call:
            print(self.name)
        if depth==0:
            return

        nc = len(self.children)
        for i, (cn, c) in enumerate(self.children.items()):
            t = type(c)
            print(prefix+f" - {cn}  ({t.__name__}, {c.get_size()})")
            if t==Dir:
                c.ls(prefix=prefix+"  ", depth=depth-1, original_call=False)

class File:

    def __init__(self, name, size):
        self.name : str = name
        self.size : int = size

    def get_size(self):
        return self.size

class DirTree:

    def __init__(self, root=None, debug=False):
        self.root : Dir = root if root is not None else Dir("/")
        self.current_dir : Dir = self.root
        self.debug = debug

    def depth_first_traverse(self):
        queue = [ self.root ]
        while len(queue)>0:
            d = queue.pop()
            queue += d.subdirs()
            yield d

    def travel(self, dirname, mk=False):
        if not dirname in self.current_dir.children:
            if mk:
                self.mkdir(dirname)
            else:
                print(f"{dirname} not a child of {self.current_dir.name}")
                return

        d = self.current_dir.children[dirname]
        if type(d)==Dir:
            self.current_dir=d
            if self.debug: print(f"moving to {d.name}")
        else:
            print(f"{dirname} not a dir in {self.current_dir.name}")

    def travel_up(self):
        if self.current_dir.parent is not None:
            self.current_dir = self.current_dir.parent
            if self.debug: print(f"moving to {self.current_dir.name}")
        else:
            print("Already at root, cannot move up")

    def goto_root(self):
        self.current_dir = self.root
        if self.debug: print("moving to /")

    def mkdir(self, name):
        if not name in self.current_dir.children:
            new_dir = Dir(name, parent=self.current_dir)
            self.current_dir.children[name] = new_dir
            if self.debug: print(f"making dir {name} in {self.current_dir.name}")
        else:
            print(f"{name} already exists in {self.current_dir.name}")

    def mkfile(self, name, *args, **kwargs):
        if not name in self.current_dir.children:
            self.current_dir.children[name] = File(name, *args, **kwargs)
            if self.debug: print(f"making file {name} in {self.current_dir.name}")
        else:
            print(f"{name} already exists in {self.current_dir.name}")


cd_reg = r"\$ cd (.*)"
dir_reg = r"dir (.*)"
file_reg = r"(\d*) (.*)"
def parse(lines, debug=False):
    tree = DirTree(debug=debug)
    for line in lines:
        if line[0]=="$":
            cd_matches = re.findall(cd_reg, line)
            if len(cd_matches)>0:
                d = cd_matches[0]
                if d=="/":
                    tree.goto_root()
                elif d=="..":
                    tree.travel_up()
                else:
                    tree.travel(d, mk=True)
            # otherwise listing will commence from next line, no need to do anything
        else:
            dir_matches = re.findall(dir_reg, line)
            file_matches = re.findall(file_reg, line) # would be nicer to only calc if needed
            if len(dir_matches)>0:
                name = dir_matches[0]
                tree.mkdir(name)
            elif len(file_matches)>0:
                size, name = file_matches[0]
                tree.mkfile(name, int(size))

    return tree

def solve(tree):
    candidates = filter(lambda d: d.get_size()<=100000, tree.depth_first_traverse())
    return sum([d.get_size() for d in candidates])

TOTAL_CAPACITY = 70000000
NEEDED_FREE    = 30000000
REQ_CAP = TOTAL_CAPACITY - NEEDED_FREE
def solve2(tree):
    MIN_TO_FREE = tree.root.get_size() - REQ_CAP
    candidates = filter(lambda d: d.size>=MIN_TO_FREE, tree.depth_first_traverse())
    return min(candidates, key=lambda d: d.size).size


test_tree = parse(test_lines)#, debug=True)
tree = parse(lines)#, debug=True)

print("Sum of at most 100000:", solve(test_tree))
print("Sum of at most 100000:", solve(tree))

# print()
# print(solve2(test_tree))
# print(solve2(tree))

