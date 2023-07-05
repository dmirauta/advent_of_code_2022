use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug)]
struct NodeMetadata<K> {
    expanded: bool,
    dist: u32,
    previous: Option<K>,
}

impl<K> NodeMetadata<K> {
    fn new() -> Self {
        NodeMetadata {
            expanded: false,
            dist: u32::MAX,
            previous: None,
        }
    }
}

/// Dijkstra's algorithm on equal length edges (where distance increases with number of hops)
pub struct FloodFill<K> {
    metadata: HashMap<K, NodeMetadata<K>>,
}

impl<K> FloodFill<K>
where
    K: Eq + Hash + Clone + Copy + Debug,
{
    pub fn new(start: K, edges: &HashMap<K, Vec<K>>) -> Self {
        let metadata: HashMap<K, _> = edges.keys().map(|k| (*k, NodeMetadata::new())).collect();
        let mut new_ff = FloodFill { metadata };
        new_ff.fill(start, edges);
        new_ff
    }

    fn fill(&mut self, start: K, edges: &HashMap<K, Vec<K>>) {
        self.metadata.get_mut(&start).unwrap().dist = 0;

        let mut queue: VecDeque<K> = VecDeque::from(vec![start]);
        while let Some(current) = queue.pop_front() {
            self.metadata.get_mut(&current).unwrap().expanded = true;
            let cd = self.metadata[&current].dist;

            for neighbour_key in edges[&current].iter() {
                let nn = self.metadata.get_mut(neighbour_key).unwrap();
                if !nn.expanded {
                    queue.push_back(neighbour_key.clone());
                    if nn.dist > cd + 1 {
                        nn.dist = cd + 1;
                        nn.previous = Some(current);
                    }
                }
            }
        }
    }

    /// Backtrace shortest path
    pub fn path_to(&self, end: K) -> Vec<K> {
        let mut path: Vec<K> = vec![end];
        while let Some(ppk) = self.metadata[path.last().unwrap()].previous.clone() {
            path.push(ppk);
        }
        path.reverse();
        path
    }

    pub fn dist(&self, end: K) -> u32 {
        self.metadata[&end].dist
    }
}
