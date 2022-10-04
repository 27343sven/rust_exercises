use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::once;
use std::ops::Range;
use std::{thread, time};

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
#[derive(Debug)]
pub enum GraphError {
    NodeInBacktraceButNoValue,
    NoValue,
    RangeHasNoMinimum,
    RangeHasNoMaximum,
    NoValidValues,
    Impossible,
    NoRoot,
    CalcValueUnavailable(u32),
    TakesPrecidence(u32),
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
    pub fn solve(&mut self, id: NId) -> Result<u32, GraphError> {
        let mut visited = HashSet::new();
        let mut backtrace = HashSet::new();
        let mut force_change = false;

        let res = self.solve_node(id, &mut visited, &mut backtrace, &mut force_change, id);
        match res {
            Err(err) => {
                println!("[!] {:?}", err);
                Err(err)
            }
            Ok(n) => Ok(n),
        }
    }

    fn solve_node(
        &mut self,
        id: NId,
        visited: &mut HashSet<NId>,
        backtrace: &mut HashSet<NId>,
        force_change: &mut bool,
        root: NId,
    ) -> Result<u32, GraphError> {
        println!("[{:<3}] Solving {}", id, id);
        thread::sleep(time::Duration::from_millis(100));
        let mut node = self.nodes.get(&id).cloned().unwrap();
        let default = Vec::new();
        let edges = self.edges.get(&id).unwrap_or(&default);
        let res = if visited.contains(&id) || edges.is_empty() {
            let (value_free, is_in_range, _) =
                self.test_value(node.v, &node, visited, root, Some(id))?;
            if value_free && is_in_range {
                if *force_change {
                    println!(
                        "[{:<3}] Existing value works, but increase change for {}",
                        id, id
                    );
                    *force_change = false;
                } else {
                    println!(
                        "[{:<3}] {} = {:?} already works, continuing...",
                        id,
                        id,
                        node.v.unwrap()
                    );
                    backtrace.insert(id);
                    return Ok(node.v.ok_or(GraphError::NoValue)?);
                }
            }
            self.guess_node(visited, backtrace, id, node.v, root)
        } else {
            if *force_change {
                if let Some(node) = self.nodes.get_mut(&id){
                    println!("[{:<3}] Clearing node {} due to force change", id, id);
                    node.v = None;
                }
            }
            self.calc_node(id, visited, backtrace, force_change, root)
        };
        return match res {
            Ok(n) | Err(GraphError::TakesPrecidence(n)) => {
                node.v = Some(n);
                self.nodes.insert(id, node);
                backtrace.insert(id);
                for (id, test_node) in self.nodes.iter() {
                    println!("\t{:5}: {:?}", id, test_node.v);
                }
                res
            }
            Err(GraphError::CalcValueUnavailable(n)) => {
                println!(
                    "[{:<3}] Calculated value {} = '{:?}' is not available, forcing change... {:?}, {:?}",
                    id, id, n, backtrace, visited
                );
                *force_change = true;
                backtrace.clear();
                self.solve_node(id, visited, backtrace, force_change, root)
            }
            Err(_) => res,
        };
    }

    fn clear_backtrace_values(&mut self, id: NId, root: NId, backtrace: &HashSet<NId>) {
        for (conn_id, m_node) in self
            .nodes
            .iter_mut()
            .filter(|(&n_id, _)| backtrace.contains(&n_id))
        {
            println!("[{:<3}]\t Clearing node '{}'.", id, conn_id);
            m_node.v = None;
        }
    }

    fn guess_node(
        &mut self,
        visited: &HashSet<NId>,
        backtrace: &HashSet<NId>,
        id: NId,
        n: Option<u32>,
        root: NId,
    ) -> Result<u32, GraphError> {
        println!("[{:<3}] Guessing node {}", id, id);
        let mut found: HashSet<u32> = HashSet::new();
        let mut first = true;
        let node = self.nodes.get(&id).cloned().unwrap();
        // start loop
        loop {
            // get a new guess
            let n = match n {
                Some(x) => Some(
                    (x + 1)
                        % (node.r.clone().max().ok_or(GraphError::RangeHasNoMaximum)?
                            + if first { 0 } else { 1 }),
                ),
                None => Some(node.r.clone().min().ok_or(GraphError::RangeHasNoMinimum)?),
            };
            if first {
                first = false;
            }
            let (mut value_free, is_in_range, takes_precidence) =
                self.test_value(n, &node, &visited, root, Some(id))?;

            // if value is not free but this node takes priority over the others clear the other nodes

            println!(
                "[{:<3}] guess: {} = {:?}; [{:?}, {:?}]; is_free: {:5}; in_range: {:5}; precidence: {:5}",
                id,
                id,
                n.unwrap(),
                node.r.clone().min().unwrap_or(0),
                node.r.clone().max().unwrap_or(0),
                value_free,
                is_in_range,
                takes_precidence,
            );
            if takes_precidence {
                println!(
                    "[{:<3}] Clearing values (Value '{:?}' not free but node takes precidence)...",
                    id, n
                );
                self.clear_backtrace_values(id, root, backtrace);
                value_free = true;
            }
            if !is_in_range || !value_free {
                if found.contains(&n.unwrap()) {
                    println!("[{:<3}] {} all possibilities tested; {:?}", id, id, found);
                    break Err(GraphError::NoValidValues);
                }
                found.insert(n.unwrap());
                println!(
                    "[{:<3}] {} = {} failed, continuing...",
                    id,
                    id,
                    n.ok_or(GraphError::NoValue)?
                );
                continue;
            }
            println!(
                "[{:<3}] {} = {} found!",
                id,
                id,
                n.ok_or(GraphError::NoValue)?
            );
            // break n.ok_or(GraphError::NoValue);
            let value = n.ok_or(GraphError::NoValue)?;
            break (!takes_precidence)
                .then_some(value)
                .ok_or(GraphError::TakesPrecidence(value));
        }
    }

    fn calc_node(
        &mut self,
        id: NId,
        visited: &mut HashSet<NId>,
        backtrace: &mut HashSet<NId>,
        force_change: &mut bool,
        root: NId,
    ) -> Result<u32, GraphError> {
        let node = self.nodes.get(&id).cloned().unwrap();
        let edges = self.edges.get(&id).cloned().unwrap_or(Vec::new());
        visited.insert(id);

        let mut v: u32;
        let mut results: Vec<u32> = Vec::new();
        let mut found: HashSet<u32> = HashSet::new();

        return 'outer: loop {
            // loop over all edges outward edges
            for (i, (n_id, e)) in edges.iter().enumerate() {
                // if backtrace already contains a connected node take it's value and multiply it by edge weight
                // else call solve_node and multiply by edge weight
                let res = if backtrace.contains(n_id) && !*force_change {
                    self.nodes
                        .get(n_id)
                        .and_then(|n_node| n_node.v.and_then(|n| Some(e * n)))
                        .ok_or(GraphError::NodeInBacktraceButNoValue)
                } else {
                    self.solve_node(n_id.clone(), visited, backtrace, force_change, root)
                        .and_then(|n| Ok(n * e))

                    // Some(self.solve_node(n_id.clone(), visited, backtrace, root)? * e)
                };

                // if the node couldn't be solved clear the backtrace
                if let Err(GraphError::TakesPrecidence(n)) = res {
                    println!("[{:<3}] Clearing backtrace...", id);
                    backtrace.clear();
                    continue 'outer;
                }

                // else add to result array
                if i >= results.len() {
                    results.push(res?);
                    continue;
                } else {
                    results[i] = res?;
                }
            }

            println!("[{:<3}] Calculating node {}", id, id);
            let n: u32 = results.iter().sum();

            v = match node.t {
                NodeType::Overflow => n / 10,
                NodeType::Variable => n % 10,
            };

            let (value_free, is_in_range, takes_precidence) =
                self.test_value(Some(v), &node, visited, root, None)?;

            println!(
                "[{:<3}] calc: {} = {}; [{}, {}] {:?}; is_free: {:5}; in_range {:5}; precidence: {:5}; {:?}...",
                id,
                id,
                v,
                node.r.clone().min().unwrap_or(0),
                node.r.clone().max().unwrap_or(0),
                edges,
                value_free,
                is_in_range,
                takes_precidence,
                visited,
            );
            for (id, test_node) in self.nodes.iter() {
                println!("\t{:5}: {:?}", id, test_node.v);
            }

            visited.remove(&id);

            break 'outer match (
                value_free,
                is_in_range,
                takes_precidence,
                found.contains(&n),
            ) {
                (_, true, _, true) => {
                    println!("[{:<3}] {} all posibilities tested; {:?}", id, id, found);
                    visited.remove(&id);
                    Ok(v)
                }
                // (_, false, _, _) => {
                //     found.insert(n);
                //     backtrace.clear();
                //     println!("[{:<3}] {} = {} not in range, continuing...", id, id, v);
                //     continue;
                // }
                (false, true, true, _) => {
                    println!( "[{:<3}] Clearing values (Value '{:?}' not free but node takes precidence)...", id, v);
                    self.clear_backtrace_values(id, root, backtrace);
                    Err(GraphError::TakesPrecidence(v))
                }
                (false, true, false, _) | (_, false, _, _) => Err(GraphError::CalcValueUnavailable(v)),
                _ => {
                    println!("[{:<3}] {} = {} found!", id, id, v);
                    Ok(v)
                }
            };
            // break 'outer (!takes_precidence)
            //     .then_some(v)
            //     .ok_or(GraphError::TakesPrecidence(v));
        };
    }

    fn test_value(
        &self,
        n: Option<u32>,
        node: &Node,
        visited: &HashSet<NId>,
        root: NId,
        exclude: Option<NId>,
    ) -> Result<(bool, bool, bool), GraphError> {
        // check if any other node has same value
        let value_free = !self
            .nodes
            .iter()
            .chain(once((
                &root,
                self.nodes.get(&root).ok_or(GraphError::NoRoot)?,
            )))
            .filter(|(&id, _)| {
                exclude
                    .and_then(|exclude_id| Some(id != exclude_id))
                    .unwrap_or(true)
            })
            .any(|(_, node)| node.v == n);
        // println!("value_free: {}", value_free);

        // test if value is in range
        let is_in_range = node.r.contains(&n.unwrap_or(10));
        // println!("[   ] Value test; {:?}; visited: {:?}", n, visited);

        // for (id, test_node) in self.nodes.iter() {
        //     println!("\t{:5}: {:?}", id, test_node.v);
        // }
        // if node is a variable and this value hasn't been used before then
        let takes_precidence = is_in_range
            && !value_free
            // && node.t == NodeType::Variable
            && visited
                .iter()
                .filter(|id| **id != root)
                .all(|id| self.nodes.get(id).unwrap().v != n);
        Ok((value_free, is_in_range, takes_precidence))
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
    fn to_go_out() {
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
                r: 0..2,
                t: NodeType::Overflow,
            },
        );
        graph.add_node(
            "c2",
            Node {
                v: None,
                r: 0..2,
                t: NodeType::Overflow,
            },
        );

        graph.add_edge("T", "O", 2);
        graph.add_edge("U", "T", 1);
        graph.add_edge("U", "G", 1);
        graph.add_edge("U", "c1", 1);
        graph.add_edge("O", "c2", 1);
        graph.add_edge("c1", "O", 2);
        graph.add_edge("c2", "G", 1);
        graph.add_edge("c2", "T", 1);
        graph.add_edge("c2", "c1", 1);

        graph.solve("O");
    }
}
