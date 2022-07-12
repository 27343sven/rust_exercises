use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub enum SolverError {
    OutOfBounds,
    UnresolvedDependencies,
    NodeNotFound,
    NoEdgesFound,
}

#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    Overflow,
    Variable,
}

pub struct Node {
    v: Option<u32>,
    r: Range<u32>,
    t: NodeType,
}

struct Graph<NId, E = (), N = ()> {
    nodes: HashMap<NId, N>,
    edges: HashMap<NId, Vec<(NId, E)>>,
    rev_edges: HashMap<NId, Vec<NId>>,
    stack: Vec<>
}

impl<NId, E, N> Graph<NId, E, N>
where
    NId: Eq + Hash + Copy + Clone,
    E: Hash,
{
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            rev_edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: NId, node: N) {
        self.nodes.insert(id, node);
    }

    pub fn add_edge(&mut self, from: NId, to: NId, edge: E) {
        self.edges
            .entry(from)
            .or_insert(Vec::new())
            .push((to, edge));
        self.rev_edges.entry(to).or_insert(Vec::new()).push(from);
    }
}

impl<NId, E> Graph<NId, E, Node>
where
    NId: Eq + Hash + Copy + Clone,
    E: Hash,
{
    pub fn solve(&mut self, id: NId) {
        let stack: NId = Vec::new();
        self.solve_node(id, stack);
    }

    fn solve_node(&mut self, id: NId, stack: &mut Vec<NId>) -> Result<u32, SolverError>{
        let test = self.calculate_node(id);
    }

    fn calculate_node(&self, id: NId) -> Result<u32, SolverError> {
        let node = self.nodes.get(&id).ok_or(SolverError::NodeNotFound)?;
        let edges = self.edges.get(&id).ok_or(SolverError::NoEdgesFound)?;

        let dependencies = edges
            .iter()
            .map(|(to, _)| self.nodes.get(to).and_then(|n| n.v))
            .collect::<Option<Vec<u32>>>()
            .ok_or(SolverError::UnresolvedDependencies)?;
        let result: u32 = dependencies.into_iter().sum();

        node.r.contains(&result).then_some(()).ok_or(SolverError::OutOfBounds)?;
        Ok(match (result, node.t) {
            (n, NodeType::Variable) => n % 10,
            (n, NodeType::Overflow) => n / 10,
        })
    }
}

mod tests {
    use std::ops::{Range, RangeBounds};

    use super::*;

    #[test]
    fn create_graph() {
        let test = 1..=9;
        let mut graph: Graph<&str, (), Node> = Graph::new();
        graph.add_node(
            "T",
            Node {
                v: None,
                r: 1..10,
                t: NodeType::Variable,
            },
        );
        graph.add_node(
            "U",
            Node {
                v: None,
                r: 0..10,
                t: NodeType::Variable,
            },
        );
        graph.add_node(
            "O",
            Node {
                v: None,
                r: 1..10,
                t: NodeType::Variable,
            },
        );
        graph.add_node(
            "G",
            Node {
                v: None,
                r: 1..10,
                t: NodeType::Variable,
            },
        );
        graph.add_node(
            "c1",
            Node {
                v: None,
                r: 0..1,
                t: NodeType::Overflow,
            },
        );
        graph.add_node(
            "c2",
            Node {
                v: None,
                r: 0..1,
                t: NodeType::Overflow,
            },
        );

        graph.add_edge("T", "O", ());
        graph.add_edge("U", "T", ());
        graph.add_edge("U", "G", ());
        graph.add_edge("U", "c1", ());
        graph.add_edge("O", "c2", ());
        graph.add_edge("c1", "T", ());
        graph.add_edge("c2", "U", ());
    }
}
