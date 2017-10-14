use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph<T, W> {
    nodes: Vec<T>,
    edges: HashMap<(usize, usize), W>
}

impl<T, W> Graph<T, W> {
    pub fn from_nodes_and_edges(nodes: Vec<T>, edges: HashMap<(usize, usize), W>) -> Graph<T, W> {
        Graph { nodes, edges }
    }

    pub fn new() -> Graph<T, W> {
        Graph::from_nodes_and_edges(Vec::new(), HashMap::new())
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn insert_node(&mut self, node: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    pub fn node(&self, i: usize) -> Option<&T> {
        self.nodes.get(i)
    }

    pub fn insert_edge(&mut self, a: usize, b: usize, x: W) {
        self.edges.insert((a, b), x);
    }

    pub fn remove_edge(&mut self, a: usize, b: usize) {
        if self.edges.contains_key(&(a, b)) {
            self.edges.remove(&(a, b));
        }
    }

    pub fn contains(&self, x: &T) -> bool where T: PartialEq {
        self.nodes.contains(x)
    }

    pub fn weight(&self, a: usize, b: usize) -> Option<&W> {
        self.edges.iter()
            .filter(|&(&(c, d), _)| a == c && b == d)
            .map(|(_, w)| w)
            .take(1)
            .collect::<Vec<&W>>()
            .get(0)
            .map(|x| *x)
    }

    pub fn set_weight(&mut self, a: usize, b: usize, w: W) {
        match self.edges.get_mut(&(a, b)) {
            Some(weight) => *weight = w,
            None => ()
        }
    }

    pub fn nodes_from(&self, i: usize) -> Vec<(usize, &T)> {
        self.edges.iter()
            .filter(|&(&(a, _), _)| a == i)
            .filter_map(|(&(_, b), _)| match self.node(b) {
                Some(v) => Some((b, v)),
                None => None
            })
            .collect()
    }

    pub fn has_edge(&self, a: usize, b: usize) -> bool {
        self.edges.contains_key(&(a, b))
    }

    pub fn find_node(&self, x: &T) -> Option<usize> where T: PartialEq {
        for (i, v) in self.nodes.iter().enumerate() {
            if v == x {
                return Some(i);
            }
        }

        None
    }
}