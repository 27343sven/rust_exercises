use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::once;
use std::ops::Range;

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub enum SolverError {
    OutOfBounds,
    UnresolvedDependencies,
    NodeNotFound,
    NoEdgesFound,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NodeType {
    Overflow,
    Variable,
}

#[derive(Clone, Debug)]
pub struct Node {
    v: Option<u32>,
    r: Range<u32>,
    t: NodeType,
}

struct Graph<NId, E = (), N = ()> {
    nodes: HashMap<NId, N>,
    edges: HashMap<NId, Vec<(NId, E)>>,
    rev_edges: HashMap<NId, Vec<NId>>,
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

impl<NId> Graph<NId, u32, Node>
where
    NId: Eq + Hash + Copy + Clone + Display + Debug,
{
    pub fn solve(&mut self, id: NId) -> Option<u32> {
        let mut visited = HashSet::new();
        let mut backtrace = HashSet::new();

        self.solve_node(id, &mut visited, &mut backtrace, id)
    }

    fn solve_node(
        &mut self,
        id: NId,
        visited: &mut HashSet<NId>,
        backtrace: &mut HashSet<NId>,
        root: NId,
    ) -> Option<u32> {
        println!("\nSolving {}", id);
        let mut node = self.nodes.get(&id).cloned().unwrap();
        let default = Vec::new();
        let edges = self.edges.get(&id).unwrap_or(&default);
        let n: Result<u32, u32> = if visited.contains(&id) || edges.is_empty() {
             self.guess_node(visited, backtrace, id, node.v, root)
        } else {
            self.calc_node(id, visited, backtrace, root).ok_or(0)
        };
      
        node.v = Some(n.unwrap_or_else(|n| n));
        self.nodes.insert(id, node);
        backtrace.insert(id);
        for (id, test_node) in self.nodes.iter() {
            println!("\t{:5}: {:?}", id, test_node.v);
        }
        n.ok()
    }

    fn guess_node(
        &mut self,
        visited: &HashSet<NId>,
        backtrace: &HashSet<NId>,
        id: NId,
        n: Option<u32>,
        root: NId,
    ) -> Result<u32, u32> {
        let mut found: HashSet<u32> = HashSet::new();
        let node = self.nodes.get(&id).cloned().unwrap();
        loop {
            let n = match n {
                Some(x) => Some((x + 1) % (node.r.clone().max().unwrap_or(9) + 1)),
                None => Some(node.r.clone().min().unwrap_or(0)),
            };
            let (mut value_free, is_in_range, takes_precidence) =
                self.test_value(n, &node, &visited, root);

            if takes_precidence {
                println!("clearing path!");
                for (_, m_node) in self
                    .nodes
                    .iter_mut()
                    .filter(|(n_id, _)| backtrace.contains(n_id) || **n_id == root)
                {
                    m_node.v = None;
                }
                value_free = true;
            }
            print!(
                "[{}] n = {:?} {:?}; is_free: {:5}; in_range: {:5}... ",
                id,
                n,
                node.r.clone().max(),
                value_free,
                is_in_range
            );
            if !is_in_range || !value_free {
                if found.contains(&n.unwrap()) {
                    println!("all possibilities tested");
                    break Ok(n.unwrap());
                }
                found.insert(n.unwrap());
                println!("failed");
                continue;
            }
            println!("found!");
            break (!takes_precidence).then_some(n.unwrap()).ok_or(n.unwrap());
        }
    }

    fn calc_node(
        &mut self,
        id: NId,
        visited: &mut HashSet<NId>,
        backtrace: &mut HashSet<NId>,
        root: NId,
    ) -> Option<u32> {
        let node = self.nodes.get(&id).cloned().unwrap();
        let edges = self.edges.get(&id).cloned().unwrap_or(Vec::new());
        visited.insert(id);

        let mut v: u32;
        let mut results: Vec<u32> = Vec::new();
        let mut found: HashSet<u32> = HashSet::new();

        v = 'outer: loop {
            for (i, (n_id, e)) in edges.iter().enumerate() {
                let res = if backtrace.contains(n_id) {
                    Some(
                        self.nodes
                            .get(n_id)
                            .and_then(|n_node| n_node.v.and_then(|n| Some(e * n)))
                            .unwrap_or(0),
                    )
                } else {
                    self.solve_node(n_id.clone(), visited, backtrace, root)
                        .and_then(|n| Some(e * n))
                };
                if res.is_none() {
                    backtrace.clear();
                    continue 'outer;
                }
                if i >= results.len() {
                    results.push(res.unwrap());
                    continue;
                } else {
                    results[i] = res.unwrap();
                }
                let n: u32 = results.iter().sum();

                v = match node.t {
                    NodeType::Overflow => n.clone() / 10,
                    NodeType::Variable => n.clone() % 10,
                };

                let (value_free, is_in_range, _) = self.test_value(Some(v), &node, visited, root);

                print!(
                    "[{}] sum = {} {:?} {:?}; is_free: {:5}; in_range {:5}...",
                    id,
                    v,
                    node.r.clone().max(),
                    edges,
                    value_free,
                    is_in_range
                );

                if !is_in_range || !value_free {
                    if found.contains(&n) {
                        println!("all possibilities tested");
                        break 'outer v;
                    }

                    found.insert(n);
                    backtrace.clear();
                    println!("failed");
                    continue;
                }
                println!("found!");

                break 'outer v;
            }
        };
        visited.remove(&id);
        Some(v)
    }

    fn test_value(
        &self,
        n: Option<u32>,
        node: &Node,
        visited: &HashSet<NId>,
        root: NId,
    ) -> (bool, bool, bool) {
        let value_free = !self
            .nodes
            .iter()
            .chain(once((
                &root,
                &Node {
                    v: n.and_then(|n| Some(n + 1)),
                    r: 1..1,
                    t: NodeType::Variable,
                },
            )))
            .any(|(_, node)| node.v == n);
        println!("value_free: {}", value_free);
        let is_in_range = node.r.contains(&n.unwrap_or(10));
        let takes_precidence = is_in_range
            && !value_free
            && node.t == NodeType::Variable
            && visited
                .iter()
                .filter(|id| **id != root)
                .all(|id| self.nodes.get(id).unwrap().v != n);
        (value_free, is_in_range, takes_precidence)
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

        node.r
            .contains(&result)
            .then_some(())
            .ok_or(SolverError::OutOfBounds)?;
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
        let mut graph: Graph<&str, u32, Node> = Graph::new();
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

        graph.add_edge("T", "O", 2);
        graph.add_edge("U", "T", 1);
        graph.add_edge("U", "G", 1);
        graph.add_edge("U", "c1", 1);
        graph.add_edge("O", "c2", 1);
        graph.add_edge("c1", "T", 1);
        graph.add_edge("c2", "U", 1);

        graph.solve("O");
    }
}
