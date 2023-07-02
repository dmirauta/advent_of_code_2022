use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug)]
struct NodeMetadata<K> {
    visited: bool,
    dist: u32,
    previous: Option<K>,
}

impl<K> NodeMetadata<K> {
    fn new() -> Self {
        NodeMetadata {
            visited: false,
            dist: u32::MAX,
            previous: None,
        }
    }
}

/// Basically Dijkstra's algorithm
pub struct FloodFill<K> {
    metadata: HashMap<K, NodeMetadata<K>>,
    pathcache: HashMap<K, Vec<K>>,
}

impl<K> FloodFill<K>
where
    K: Eq + Hash + Clone + Copy + Debug,
{
    pub fn new(keys: impl Iterator<Item = K>, start: K, edges: &HashMap<K, Vec<(K, u32)>>) -> Self {
        let metadata: HashMap<K, _> = keys.map(|k| (k, NodeMetadata::new())).collect();
        let mut new_ff = FloodFill {
            metadata,
            pathcache: HashMap::new(),
        };
        new_ff.fill(start, edges);
        new_ff
    }

    fn fill(&mut self, start: K, edges: &HashMap<K, Vec<(K, u32)>>) {
        let mut queue: Vec<K> = vec![start];
        while let Some(current) = queue.pop() {
            self.metadata.get_mut(&current).unwrap().visited = true;
            let cd = self.metadata[&current].dist;

            for (neighbour_key, edge_len) in edges[&current].iter() {
                let nn = self.metadata.get_mut(neighbour_key).unwrap();
                if !nn.visited {
                    queue.push(neighbour_key.clone());
                    if nn.dist > cd + edge_len {
                        nn.dist = cd + edge_len;
                        nn.previous = Some(current);
                    }
                }
            }
        }
    }

    fn compute_path_to(&self, end: K) -> Vec<K> {
        let mut path: Vec<K> = vec![end];
        while let Some(ppk) = self.metadata[path.last().unwrap()].previous.clone() {
            path.push(ppk);
        }
        path.reverse();
        path
    }

    pub fn path_to(&mut self, end: K) -> Vec<K> {
        match self.pathcache.get(&end) {
            Some(path) => path,
            _ => {
                self.pathcache.insert(end, self.compute_path_to(end));
                &self.pathcache[&end]
            }
        }
        .clone()
    }
}
